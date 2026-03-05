# Click-to-Action Loop Mechanics

> **Status:** INTERACTION SYSTEM SPECIFICATION v1.0 | **Date:** 2026-03-04  
> **Related:** ADR-021, TACTICAL_WIRING_SPRINT.md, MAFIA_WARS_UI_STRATEGY.md

## Overview

The Click-to-Action Loop Mechanics define the core interaction patterns that enable the 30-second gameplay cycle. This system transforms complex game actions into simple, intuitive clicks while maintaining strategic depth and ensuring zero-latency response times.

## Loop Architecture

### Core Interaction Flow

```rust
pub struct ClickActionLoop {
    pub input_state: InputState,
    pub action_processor: ActionProcessor,
    pub response_handler: ResponseHandler,
    pub feedback_system: FeedbackSystem,
    pub performance_tracker: PerformanceTracker,
}

#[derive(Debug, Clone)]
pub struct InputState {
    pub last_click: Option<ClickEvent>,
    pub click_sequence: Vec<ClickEvent>,
    pub current_context: InteractionContext,
    pub input_buffer: VecDeque<InputEvent>,
}

#[derive(Debug, Clone)]
pub struct ClickEvent {
    pub position: egui::Pos2,
    pub timestamp: std::time::Instant,
    pub target: ClickTarget,
    pub modifier_keys: Vec<ModifierKey>,
    pub click_type: ClickType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClickType {
    Single,
    Double,
    Right,
    LongPress,
    Drag,
}

impl ClickActionLoop {
    pub fn process_click(&mut self, click_event: ClickEvent) -> Vec<UIAction> {
        let start_time = std::time::Instant::now();
        
        // Update input state
        self.update_input_state(click_event);
        
        // Determine action based on context
        let actions = self.determine_actions();
        
        // Process actions
        let processed_actions = self.action_processor.process(actions);
        
        // Generate feedback
        self.feedback_system.generate_feedback(&processed_actions);
        
        // Track performance
        let response_time = start_time.elapsed();
        self.performance_tracker.record_response(response_time);
        
        processed_actions
    }
    
    fn update_input_state(&mut self, click_event: ClickEvent) {
        self.last_click = Some(click_event.clone());
        self.click_sequence.push(click_event.clone());
        
        // Limit sequence length to prevent memory issues
        if self.click_sequence.len() > 10 {
            self.click_sequence.remove(0);
        }
        
        // Update context based on click
        self.current_context = self.determine_context(&click_event);
    }
    
    fn determine_actions(&self) -> Vec<UIAction> {
        match &self.current_context {
            InteractionContext::NodeSelection { node_id } => {
                vec![UIAction::OpenSquadSelection { node_id: *node_id }]
            },
            InteractionContext::SquadSelection { slime_id } => {
                vec![UIAction::ToggleSlimeSelection { slime_id: *slime_id }]
            },
            InteractionContext::Breeding { parent_id } => {
                vec![UIAction::SelectBreedingParent { parent_id: *parent_id }]
            },
            InteractionContext::Deployment { node_id, squad } => {
                vec![UIAction::DeploySquad { node_id: *node_id, squad: squad.clone() }]
            },
            InteractionContext::MenuNavigation { target_panel } => {
                vec![UIAction::SwitchPanel { panel: target_panel.clone() }]
            },
            _ => vec![UIAction::NoAction],
        }
    }
}
```

## Interaction Contexts

### Node Selection Context

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InteractionContext {
    NodeSelection { node_id: Uuid },
    SquadSelection { slime_id: Uuid },
    Breeding { parent_id: Uuid },
    Deployment { node_id: Uuid, squad: Vec<Uuid> },
    MenuNavigation { target_panel: String },
    ResourceManagement { resource_type: ResourceType },
    ResearchSelection { tech_id: Uuid },
}

impl ClickActionLoop {
    fn determine_context(&self, click_event: &ClickEvent) -> InteractionContext {
        match &click_event.target {
            ClickTarget::Node(node_id) => {
                InteractionContext::NodeSelection { node_id: *node_id }
            },
            ClickTarget::SlimeCard(slime_id) => {
                if self.is_in_squad_selection_mode() {
                    InteractionContext::SquadSelection { slime_id: *slime_id }
                } else if self.is_in_breeding_mode() {
                    InteractionContext::Breeding { parent_id: *slime_id }
                } else {
                    InteractionContext::SquadSelection { slime_id: *slime_id }
                }
            },
            ClickTarget::Incubator => {
                InteractionContext::MenuNavigation { target_panel: "incubator".to_string() }
            },
            ClickTarget::Expeditions => {
                InteractionContext::MenuNavigation { target_panel: "expeditions".to_string() }
            },
            ClickTarget::Research => {
                InteractionContext::MenuNavigation { target_panel: "research".to_string() }
            },
            ClickTarget::ResourceIcon(resource_type) => {
                InteractionContext::ResourceManagement { resource_type: resource_type.clone() }
            },
            ClickTarget::TechItem(tech_id) => {
                InteractionContext::ResearchSelection { tech_id: *tech_id }
            },
            _ => InteractionContext::NodeSelection { node_id: Uuid::new_v4() },
        }
    }
    
    fn is_in_squad_selection_mode(&self) -> bool {
        self.click_sequence
            .iter()
            .any(|click| matches!(click.target, ClickTarget::Node(_)))
    }
    
    fn is_in_breeding_mode(&self) -> bool {
        self.click_sequence
            .iter()
            .any(|click| matches!(click.target, ClickTarget::Incubator))
    }
}
```

### Squad Selection Flow

```rust
pub struct SquadSelectionFlow {
    pub active: bool,
    pub target_node: Option<Uuid>,
    pub selected_slimes: HashSet<Uuid>,
    pub max_squad_size: usize,
    pub selection_requirements: SquadRequirements,
}

#[derive(Debug, Clone)]
pub struct SquadRequirements {
    pub min_level: u8,
    pub max_cost: u64,
    pub required_cultures: Option<Vec<Culture>>,
    pub forbidden_states: Vec<OperatorState>,
}

impl SquadSelectionFlow {
    pub fn new() -> Self {
        Self {
            active: false,
            target_node: None,
            selected_slimes: HashSet::new(),
            max_squad_size: 3,
            selection_requirements: SquadRequirements::default(),
        }
    }
    
    pub fn activate_for_node(&mut self, node_id: Uuid, node: &MapNode) {
        self.active = true;
        self.target_node = Some(node_id);
        self.selected_slimes.clear();
        
        // Set requirements based on node
        self.selection_requirements = SquadRequirements {
            min_level: 2,
            max_cost: node.deployment_cost,
            required_cultures: None, // Any culture allowed
            forbidden_states: vec![
                OperatorState::Injured(chrono::Utc::now()),
                OperatorState::Deployed(Uuid::new_v4()),
            ],
        };
    }
    
    pub fn handle_slime_click(&mut self, slime_id: Uuid, slime: &SlimeGenome) -> SquadSelectionResult {
        if !self.active {
            return SquadSelectionResult::NotActive;
        }
        
        // Check if slime meets requirements
        if !self.meets_requirements(slime) {
            return SquadSelectionResult::DoesNotMeetRequirements;
        }
        
        // Toggle selection
        if self.selected_slimes.contains(&slime_id) {
            self.selected_slimes.remove(&slime_id);
            SquadSelectionResult::Deselected
        } else if self.selected_slimes.len() < self.max_squad_size {
            self.selected_slimes.insert(slime_id);
            SquadSelectionResult::Selected
        } else {
            SquadSelectionResult::SquadFull
        }
    }
    
    pub fn can_deploy(&self) -> bool {
        self.active && 
        self.target_node.is_some() && 
        !self.selected_slimes.is_empty() &&
        self.selected_slimes.len() <= self.max_squad_size
    }
    
    pub fn deploy_squad(&mut self) -> Option<DeploymentAction> {
        if self.can_deploy() {
            let node_id = self.target_node?;
            let squad: Vec<Uuid> = self.selected_slimes.iter().cloned().collect();
            
            self.reset();
            
            Some(DeploymentAction {
                node_id,
                squad,
                timestamp: chrono::Utc::now(),
            })
        } else {
            None
        }
    }
    
    fn meets_requirements(&self, slime: &SlimeGenome) -> bool {
        // Check level requirement
        if slime.level < self.selection_requirements.min_level {
            return false;
        }
        
        // Check state requirements
        if self.selection_requirements.forbidden_states.contains(&slime.state) {
            return false;
        }
        
        // Check culture requirements (if any)
        if let Some(required_cultures) = &self.selection_requirements.required_cultures {
            if !required_cultures.contains(&slime.culture) {
                return false;
            }
        }
        
        true
    }
    
    fn reset(&mut self) {
        self.active = false;
        self.target_node = None;
        self.selected_slimes.clear();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SquadSelectionResult {
    Selected,
    Deselected,
    SquadFull,
    DoesNotMeetRequirements,
    NotActive,
}

#[derive(Debug, Clone)]
pub struct DeploymentAction {
    pub node_id: Uuid,
    pub squad: Vec<Uuid>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
```

## Action Processing System

### Action Processor

```rust
pub struct ActionProcessor {
    pub action_handlers: HashMap<UIAction, Box<dyn ActionHandler>>,
    pub processing_queue: VecDeque<ProcessedAction>,
    pub error_handler: ErrorHandler,
}

impl ActionProcessor {
    pub fn new() -> Self {
        let mut handlers: HashMap<UIAction, Box<dyn ActionHandler>> = HashMap::new();
        
        // Register action handlers
        handlers.insert(UIAction::OpenSquadSelection { node_id: Uuid::new_v4() }, 
                       Box::new(SquadSelectionHandler::new()));
        handlers.insert(UIAction::DeploySquad { node_id: Uuid::new_v4(), squad: vec![] }, 
                       Box::new(DeploymentHandler::new()));
        handlers.insert(UIAction::SelectBreedingParent { parent_id: Uuid::new_v4() }, 
                       Box::new(BreedingHandler::new()));
        handlers.insert(UIAction::SwitchPanel { panel: String::new() }, 
                       Box::new(PanelSwitchHandler::new()));
        
        Self {
            action_handlers: handlers,
            processing_queue: VecDeque::new(),
            error_handler: ErrorHandler::new(),
        }
    }
    
    pub fn process(&mut self, actions: Vec<UIAction>) -> Vec<ProcessedAction> {
        let mut processed_actions = Vec::new();
        
        for action in actions {
            match self.process_single_action(action) {
                Ok(processed) => {
                    processed_actions.push(processed);
                },
                Err(error) => {
                    self.error_handler.handle_error(error);
                },
            }
        }
        
        processed_actions
    }
    
    fn process_single_action(&mut self, action: UIAction) -> Result<ProcessedAction, ActionError> {
        let handler = self.action_handlers
            .get_mut(&action)
            .ok_or(ActionError::NoHandler(action.clone()))?;
        
        handler.handle(action)
    }
}

#[derive(Debug, Clone)]
pub struct ProcessedAction {
    pub original_action: UIAction,
    pub result: ActionResult,
    pub processing_time: Duration,
    pub success: bool,
}

#[derive(Debug, Clone)]
pub enum ActionResult {
    SquadSelectionOpened { node_id: Uuid },
    SquadSelectionUpdated { slime_id: Uuid, selected: bool },
    DeploymentInitiated { node_id: Uuid, squad: Vec<Uuid> },
    BreedingParentSelected { parent_id: Uuid, slot: usize },
    PanelSwitched { new_panel: String },
    Error { error_message: String },
}
```

### Feedback System

```rust
pub struct FeedbackSystem {
    pub visual_feedback: VisualFeedback,
    pub audio_feedback: AudioFeedback,
    pub haptic_feedback: HapticFeedback,
    pub notification_system: NotificationSystem,
}

impl FeedbackSystem {
    pub fn generate_feedback(&mut self, actions: &[ProcessedAction]) {
        for action in actions {
            if action.success {
                self.generate_success_feedback(&action.result);
            } else {
                self.generate_error_feedback(&action.result);
            }
        }
    }
    
    fn generate_success_feedback(&mut self, result: &ActionResult) {
        match result {
            ActionResult::SquadSelectionOpened { .. } => {
                self.visual_feedback.show_highlight(HighlightType::NodeSelection);
                self.audio_feedback.play_sound(SoundType::Click);
            },
            ActionResult::SquadSelectionUpdated { selected, .. } => {
                if *selected {
                    self.visual_feedback.show_highlight(HighlightType::SlimeSelected);
                    self.audio_feedback.play_sound(SoundType::Select);
                } else {
                    self.visual_feedback.show_highlight(HighlightType::SlimeDeselected);
                    self.audio_feedback.play_sound(SoundType::Deselect);
                }
            },
            ActionResult::DeploymentInitiated { .. } => {
                self.visual_feedback.show_animation(AnimationType::Deployment);
                self.audio_feedback.play_sound(SoundType::Deploy);
                self.haptic_feedback.vibrate(HapticPattern::Short);
                self.notification_system.show_notification(
                    NotificationType::Success,
                    "Squad deployed successfully!"
                );
            },
            ActionResult::BreedingParentSelected { .. } => {
                self.visual_feedback.show_highlight(HighlightType::BreedingSlot);
                self.audio_feedback.play_sound(SoundType::Select);
            },
            ActionResult::PanelSwitched { .. } => {
                self.visual_feedback.show_transition(TransitionType::Instant);
                self.audio_feedback.play_sound(SoundType::Switch);
            },
            ActionResult::Error { .. } => {
                self.generate_error_feedback(result);
            },
        }
    }
    
    fn generate_error_feedback(&mut self, result: &ActionResult) {
        if let ActionResult::Error { error_message } = result {
            self.visual_feedback.show_highlight(HighlightType::Error);
            self.audio_feedback.play_sound(SoundType::Error);
            self.haptic_feedback.vibrate(HapticPattern::Error);
            self.notification_system.show_notification(
                NotificationType::Error,
                error_message
            );
        }
    }
}

#[derive(Debug, Clone)]
pub enum HighlightType {
    NodeSelection,
    SlimeSelected,
    SlimeDeselected,
    BreedingSlot,
    Error,
}

#[derive(Debug, Clone)]
pub enum AnimationType {
    Deployment,
    Breeding,
    Research,
    ResourceCollection,
}

#[derive(Debug, Clone)]
pub enum TransitionType {
    Instant,
    Slide,
    Fade,
}
```

## Performance Optimization

### Response Time Tracking

```rust
pub struct PerformanceTracker {
    pub response_times: VecDeque<Duration>,
    pub click_counts: VecDeque<u32>,
    pub error_rates: VecDeque<f32>,
    pub frame_rates: VecDeque<f32>,
    pub max_history_size: usize,
}

impl PerformanceTracker {
    pub fn new() -> Self {
        Self {
            response_times: VecDeque::with_capacity(1000),
            click_counts: VecDeque::with_capacity(1000),
            error_rates: VecDeque::with_capacity(1000),
            frame_rates: VecDeque::with_capacity(1000),
            max_history_size: 1000,
        }
    }
    
    pub fn record_response(&mut self, response_time: Duration) {
        self.response_times.push_back(response_time);
        
        // Maintain history size
        if self.response_times.len() > self.max_history_size {
            self.response_times.pop_front();
        }
    }
    
    pub fn record_click(&mut self) {
        let current_count = self.click_counts.back().copied().unwrap_or(0);
        self.click_counts.push_back(current_count + 1);
        
        if self.click_counts.len() > self.max_history_size {
            self.click_counts.pop_front();
        }
    }
    
    pub fn get_average_response_time(&self) -> Duration {
        if self.response_times.is_empty() {
            return Duration::ZERO;
        }
        
        let total: Duration = self.response_times.iter().sum();
        total / self.response_times.len() as u32
    }
    
    pub fn get_actions_per_minute(&self) -> f32 {
        if self.click_counts.len() < 2 {
            return 0.0;
        }
        
        let recent_clicks: u32 = self.click_counts.iter().rev().take(60).sum();
        recent_clicks as f32
    }
    
    pub fn is_performance_acceptable(&self) -> bool {
        let avg_response = self.get_average_response_time();
        let apm = self.get_actions_per_minute();
        
        avg_response < Duration::from_millis(100) && apm >= 120.0
    }
}
```

### Input Optimization

```rust
pub struct InputOptimizer {
    pub click_debounce_time: Duration,
    pub double_click_threshold: Duration,
    pub drag_threshold: f32,
    pub long_press_threshold: Duration,
}

impl InputOptimizer {
    pub fn new() -> Self {
        Self {
            click_debounce_time: Duration::from_millis(50),
            double_click_threshold: Duration::from_millis(300),
            drag_threshold: 5.0,
            long_press_threshold: Duration::from_millis(500),
        }
    }
    
    pub fn should_process_click(&self, current_click: &ClickEvent, last_click: &Option<ClickEvent>) -> bool {
        if let Some(last) = last_click {
            let time_since_last = current_click.timestamp.duration_since(last.timestamp);
            
            // Debounce rapid clicks
            if time_since_last < self.click_debounce_time {
                return false;
            }
            
            // Check for double click
            if time_since_last < self.double_click_threshold {
                let distance = current_click.position.distance(last.position);
                if distance < self.drag_threshold {
                    return false; // This is a double click, handle separately
                }
            }
        }
        
        true
    }
    
    pub fn classify_click(&self, click_event: &ClickEvent, click_sequence: &[ClickEvent]) -> ClickType {
        // Check for double click
        if let Some(last_click) = click_sequence.last() {
            let time_since_last = click_event.timestamp.duration_since(last_click.timestamp);
            let distance = click_event.position.distance(last_click.position);
            
            if time_since_last < self.double_click_threshold && distance < self.drag_threshold {
                return ClickType::Double;
            }
        }
        
        // Check for long press (would need to be handled at input level)
        // For now, default to single click
        ClickType::Single
    }
}
```

## Mobile Adaptation

### Touch Interface

```rust
pub struct TouchInterface {
    pub touch_targets: Vec<TouchTarget>,
    pub gesture_recognizer: GestureRecognizer,
    pub touch_feedback: TouchFeedback,
}

#[derive(Debug, Clone)]
pub struct TouchTarget {
    pub id: String,
    pub bounds: egui::Rect,
    pub action: UIAction,
    pub min_size: egui::Vec2,
    pub visual_feedback: TouchFeedbackType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TouchFeedbackType {
    Highlight,
    Vibration,
    Sound,
    Animation,
}

impl TouchInterface {
    pub fn new() -> Self {
        Self {
            touch_targets: Vec::new(),
            gesture_recognizer: GestureRecognizer::new(),
            touch_feedback: TouchFeedback::new(),
        }
    }
    
    pub fn register_touch_target(&mut self, target: TouchTarget) {
        // Ensure minimum touch size (44x44px)
        let adjusted_bounds = self.ensure_minimum_size(target.bounds, target.min_size);
        
        self.touch_targets.push(TouchTarget {
            bounds: adjusted_bounds,
            ..target
        });
    }
    
    fn ensure_minimum_size(&self, bounds: egui::Rect, min_size: egui::Vec2) -> egui::Rect {
        let width = bounds.width().max(min_size.x);
        let height = bounds.height().max(min_size.y);
        
        let center = bounds.center();
        let half_size = egui::Vec2::new(width / 2.0, height / 2.0);
        
        egui::Rect::from_center_size(center, half_size * 2.0)
    }
    
    pub fn handle_touch(&mut self, touch_pos: egui::Pos2) -> Option<UIAction> {
        // Find touched target
        let touched_target = self.touch_targets
            .iter()
            .find(|target| target.bounds.contains(touch_pos));
        
        if let Some(target) = touched_target {
            // Generate touch feedback
            self.touch_feedback.provide_feedback(&target.visual_feedback);
            
            Some(target.action.clone())
        } else {
            None
        }
    }
}
```

## Validation Criteria

### Core Loop Performance

```rust
pub struct LoopValidation {
    pub response_time_target: Duration,
    pub actions_per_minute_target: f32,
    pub error_rate_threshold: f32,
    pub frame_rate_target: f32,
}

impl LoopValidation {
    pub fn validate_click_loop(&self, tracker: &PerformanceTracker) -> ValidationResult {
        let avg_response = tracker.get_average_response_time();
        let apm = tracker.get_actions_per_minute();
        let error_rate = self.calculate_error_rate(tracker);
        let frame_rate = self.get_average_frame_rate(tracker);
        
        ValidationResult {
            response_time_ok: avg_response <= self.response_time_target,
            apm_ok: apm >= self.actions_per_minute_target,
            error_rate_ok: error_rate <= self.error_rate_threshold,
            frame_rate_ok: frame_rate >= self.frame_rate_target,
            overall_ok: avg_response <= self.response_time_target &&
                        apm >= self.actions_per_minute_target &&
                        error_rate <= self.error_rate_threshold &&
                        frame_rate >= self.frame_rate_target,
        }
    }
}
```

The Click-to-Action Loop Mechanics provide the foundation for responsive, intuitive interactions that enable the 30-second gameplay cycle while maintaining the strategic depth required for engaging gameplay. This system ensures every click is meaningful and immediately responsive, creating the addictive core loop that defines the Mafia Wars experience.
