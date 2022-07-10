
#[derive(PartialEq, Copy, Clone)]
pub enum ElysiusEventType {
    InitShipTransfer,     
    TestEvent,
    LeftMouseDown,
    RightMouseDown,
    NewMenu,
}
//Comment to push
#[derive(Copy, Clone)]
pub struct Event {
    event_type: ElysiusEventType,
    generated_by: Option<usize>,
    target: Option<usize>,
}

impl Event {
    pub fn new(event_type: ElysiusEventType, generated_by: Option<usize>, target: Option<usize>) -> Self {
        Event {event_type, generated_by, target}
    }
    pub fn event_type(&self) -> ElysiusEventType {return self.event_type;}
    pub fn generated_by(&self) -> Option<usize> {return self.generated_by;}
    pub fn target(&self) -> Option<usize> {return self.target;}
    pub fn is_event(&self, event_type: ElysiusEventType) -> bool {
        if event_type == self.event_type {return true;} 
        else {return false;}
    }
    
}


pub struct EventSystem {
    events: Vec<Event>,
}


impl EventSystem {
    pub fn new() -> Self {
        EventSystem { events: Vec::new(),}
    }

    pub fn new_event(
        self: &mut Self, 
        event_type: ElysiusEventType, 
        gen_by: Option<usize>, 
        target: Option<usize>
    ) {
        //Make a new event and push it to event system
        self.events.push(Event::new(event_type, gen_by, target));
    }
    //add a new event if it is only the enum, everyhting else will be filled none
    pub fn new_event_ez(self: &mut Self, event_type: ElysiusEventType) {
        self.events.push(Event::new(event_type,None,None));
    }
    //Create a new event from an already created event struct
    pub fn new_event_from(self: &mut Self, event: Event) {
        self.events.push(event);
    }
    pub fn clear_events(self: &mut Self) {self.events.clear();}
    
    pub fn get_events(&self, e_type: ElysiusEventType) -> Vec<Event> {
        //fnction should collect all of the events that match event_type given
        let new_events = self.events.iter()
            .filter(|e| e.is_event(e_type)).cloned()
            .collect::<Vec<Event>>();
        return new_events; 
    }
    //function will return true if any of the events match the given event
    pub fn check_event(&self, event_type: ElysiusEventType) -> bool {
        if self.events.iter().any(|&e| e.is_event(event_type)) {return true;}
        else {return false;}
    } 
}


//0-------------------TESTS----------------------------------------------0

#[cfg(test)]
mod tests {
    use super::{EventSystem, Event, ElysiusEventType};

    #[test]
    fn check_if_tests_work() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_get_events() {
        let e1 = Event::new(ElysiusEventType::InitShipTransfer, Some(2), Some(3));
        let e2 = Event::new(ElysiusEventType::InitShipTransfer, Some(5), Some(4));
        let e3 = Event::new(ElysiusEventType::TestEvent, None, None);
        let mut event_system = EventSystem::new();
        event_system.new_event_from(e1);
        event_system.new_event_from(e2);
        event_system.new_event_from(e3);
        event_system.new_event_from(e3);
        let v = event_system.get_events(ElysiusEventType::InitShipTransfer);
        //Vector.len() returns the intager length not a i-1 length
        assert_eq!(v.len(), 2);
    }


}










