
#[derive(PartialEq, Copy, Clone)]
pub enum EventType {
    InitShipTransfer,     
    ShipTransferComplete,
    MoveShip,
    TestEvent,
    LeftMouseDown,
    RightMouseDown,
    NewMenu,
}

impl EventType {
    pub fn is_persistant(&self) -> bool {
        match self {
            EventType::InitShipTransfer => {return true;}
            _ => {return false;}
        }
    }
}

//Comment to push
#[derive(Copy, Clone)]
pub struct Event {
    event_type: EventType,
    generated_by: Option<usize>,
    target: Option<usize>,
}


impl Event {
    pub fn new(event_type: EventType, generated_by: Option<usize>, target: Option<usize>) -> Self {
        Event {event_type, generated_by, target}
    }
    pub fn event_type(&self) -> EventType {return self.event_type;}
    pub fn generated_by(&self) -> Option<usize> {return self.generated_by;}
    pub fn target(&self) -> Option<usize> {return self.target;}
    pub fn is_event(&self, event_type: EventType) -> bool {
        if event_type == self.event_type {return true;} 
        else {return false;}
    }
    pub fn is_persistant(&self) -> bool {return self.event_type.is_persistant();}
    pub fn set_target(self: &mut Self, id: usize) {self.target = Some(id);}
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
        event_type: EventType, 
        gen_by: Option<usize>, 
        target: Option<usize>
    ) {
        //Make a new event and push it to event system
        self.events.push(Event::new(event_type, gen_by, target));
    }
    //add a new event if it is only the enum, everyhting else will be filled none
    pub fn new_event_ez(self: &mut Self, event_type: EventType) {
        self.events.push(Event::new(event_type,None,None));
    }
    //Create a new event from an already created event struct
    pub fn new_event_from(self: &mut Self, event: Event) {
        self.events.push(event);
    }
    //Will clear all events that are not persistant
    pub fn clear_events(self: &mut Self) {
        self.events.retain(|e| e.is_persistant());
    }
    //Will clear any event of submitted type from the event system
    pub fn clear_event_type(self: &mut Self, event_type: EventType) {
        self.events.retain(|e| !e.is_event(event_type));
    }
    
    pub fn get_events(&self, e_type: EventType) -> Vec<Event> {
        //fnction should collect all of the events that match event_type given
        let new_events = self.events.iter()
            .filter(|e| e.is_event(e_type)).cloned()
            .collect::<Vec<Event>>();
        return new_events; 
    }
    //function will return true if any of the events match the given event
    pub fn check_event(&self, event_type: EventType) -> bool {
        if self.events.iter().any(|&e| e.is_event(event_type)) {return true;}
        else {return false;}
    } 

    pub fn update_event_target(
        self: &mut Self, 
        event_type: EventType, 
        target: usize
    ) {
        for i in 0..self.events.len() {
            if self.events[i].is_event(event_type) {
                self.events[i].set_target(target);
            }
        }       
    }

    fn num_of_events(&self) -> usize {return self.events.len();}
}

//0-------------------TESTS----------------------------------------------0
//TODO Push Dev Ahead of Master


#[cfg(test)]
mod tests {
    use super::{EventSystem, Event, EventType};

    #[test]
    fn check_if_tests_work() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_get_events() {
        let e1 = Event::new(EventType::InitShipTransfer, Some(2), Some(3));
        let e2 = Event::new(EventType::InitShipTransfer, Some(5), Some(4));
        let e3 = Event::new(EventType::TestEvent, None, None);
        let mut event_system = EventSystem::new();
        event_system.new_event_from(e1);
        event_system.new_event_from(e2);
        event_system.new_event_from(e3);
        event_system.new_event_from(e3);
        let v = event_system.get_events(EventType::InitShipTransfer);
        //Vector.len() returns the intager length not a i-1 length
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn test_persistance() {
        let e1 = Event::new(EventType::InitShipTransfer, Some(2), Some(3));
        let e3 = Event::new(EventType::TestEvent, None, None);
        let mut event_system = EventSystem::new();
        event_system.new_event_from(e1);
        event_system.new_event_from(e1);
        event_system.new_event_from(e3);
        event_system.new_event_from(e3);
        event_system.clear_events();
        //Vector.len() returns the intager length not a i-1 length
        assert_eq!(event_system.num_of_events(), 2);
    }
}










