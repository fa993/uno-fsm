use core::fmt;
use std::{fmt::Debug, fmt::Write, panic};

use rand::Rng;
use wasm_bindgen::prelude::*;

pub trait State {
    type Input;
    type Output;
    type Error;

    fn validate(&self, input: &Self::Input) -> Result<(), Self::Error>;

    fn compute(&self, input: &Self::Input) -> Option<Self::Output>;

    fn transition(&mut self, input: &Self::Input);

    fn next(
        &mut self,
        transition_event: &Self::Input,
    ) -> Result<Option<Self::Output>, Self::Error> {
        self.validate(transition_event)?;
        let out = self.compute(transition_event);
        self.transition(transition_event);
        Ok(out)
    }
}

//a player id type, here usize for demo
type PlayerId = usize;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CardType {
    Red = 0,
    Blue = 1,
    Green = 2,
    Yellow = 3,
    // if one wants to implement special cards such as draw 4s and wild,
    // one would put it here
}

//every card is represented by a struct containing CardType(just the colour for now)
//and the number,
//if you want to include reverse, skip, etc
//you would need to represent another enum for the number
#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone)]
pub struct UnoCard {
    card_type: CardType,
    number: usize,
}

//every uno event has an associated player id, i.e the player who sent that event
//the event also has an event type
pub struct UnoEvent {
    id: PlayerId,
    event_type: UnoEventType,
}

//the event type which describes what sort of event it is
//the input type for this state
#[derive(PartialEq)]
pub enum UnoEventType {
    //the discard event also has a card as a value, indicating the card to be discarded
    Discard(UnoCard),
    //event sent when user has no available card to withdraw,
    //to prepare the engine for a withdraw
    NoCard,
    //event sent when user actually wants to draw a card
    Draw,
}

//the error type for this state
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum UnoError {
    //used when the discarded card is not of the same colour or the same number
    IncorrectCard,
    //used when the expected player did not send the event
    IncorrectPlayer,
    UnexpectedEvent,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum UnoStateType {
    WaitingForDiscard = 0,
    WaitingForDraw = 1,
}

//the output type
#[derive(Debug)]
pub enum UnoOutput {
    //sent after a draw event with the drawn card
    Card(UnoCard),
}

//the state type
#[wasm_bindgen]
#[derive(Debug)]
pub struct UnoGameState {
    //other fields to indicate deck size, maybe shuffled deck or card order
    //basically other metadata fields
    players_num: usize,
    //the card on top of the deck
    top_card: UnoCard,
    state_type: UnoStateType,
    //the player who is expected to send the next event
    expected_player_turn: PlayerId,
}

impl State for UnoGameState {
    type Input = UnoEvent;
    type Output = UnoOutput;
    type Error = UnoError;

    fn validate(&self, t: &Self::Input) -> Result<(), UnoError> {
        if self.expected_player_turn != t.id {
            return Err(UnoError::IncorrectPlayer);
        }
        match (&self.state_type, &t.event_type) {
            (UnoStateType::WaitingForDiscard, UnoEventType::Discard(c)) => {
                if self.top_card.card_type == c.card_type || self.top_card.number == c.number {
                    Ok(())
                } else {
                    Err(UnoError::IncorrectCard)
                }
            }
            (UnoStateType::WaitingForDiscard, UnoEventType::NoCard) => Ok(()),
            (UnoStateType::WaitingForDraw, UnoEventType::Draw) => Ok(()),
            _ => Err(UnoError::UnexpectedEvent),
        }
    }

    fn compute(&self, t: &UnoEvent) -> Option<UnoOutput> {
        if self.state_type == UnoStateType::WaitingForDraw && t.event_type == UnoEventType::Draw {
            let y = match rand::thread_rng().gen_range(1..=4) {
                1 => CardType::Red,
                2 => CardType::Green,
                3 => CardType::Yellow,
                4 => CardType::Blue,
                _ => panic!("Random out of range"),
            };
            Some(UnoOutput::Card(UnoCard {
                card_type: y,
                number: rand::thread_rng().gen_range(1..10),
            }))
        } else {
            None
        }
    }

    fn transition(&mut self, input: &Self::Input) {
        match (&self.state_type, &input.event_type) {
            (UnoStateType::WaitingForDiscard, UnoEventType::Discard(c)) => {
                self.top_card = c.clone();
                self.expected_player_turn = (self.expected_player_turn + 1) % self.players_num;
            }
            (UnoStateType::WaitingForDiscard, UnoEventType::NoCard) => {
                self.state_type = UnoStateType::WaitingForDraw;
            }
            (UnoStateType::WaitingForDraw, UnoEventType::Draw) => {
                self.state_type = UnoStateType::WaitingForDiscard;
            }
            _ => {}
        }
    }
}

#[wasm_bindgen]
impl UnoGameState {
    pub fn new() -> UnoGameState {
        UnoGameState {
            players_num: 4,
            top_card: UnoCard {
                card_type: CardType::Blue,
                number: 5,
            },
            state_type: UnoStateType::WaitingForDiscard,
            expected_player_turn: 0,
        }
    }

    pub fn draw(&mut self, id: u32) -> String {
        let mut r = String::new();
        let ret = self.next(&UnoEvent {
            id: id.try_into().expect("Value not fit"),
            event_type: UnoEventType::Draw,
        });
        write!(r, "{:?}", ret).expect("Failed to write O/P");
        r
    }

    pub fn no_card(&mut self, id: u32) -> String {
        let mut r = String::new();
        let ret = self.next(&UnoEvent {
            id: id.try_into().expect("Value not fit"),
            event_type: UnoEventType::NoCard,
        });
        write!(r, "{:?}", ret).expect("Failed to write O/P");
        r
    }

    pub fn discard(&mut self, id: u32, color: CardType, num: u32) -> String {
        let mut r = String::new();
        let ret = self.next(&UnoEvent {
            id: id.try_into().expect("Value not fit"),
            event_type: UnoEventType::Discard(UnoCard {
                card_type: color,
                number: num.try_into().expect("Value not fit"),
            }),
        });
        write!(r, "{:?}", ret).expect("Failed to write O/P");
        r
    }

    pub fn current_state(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for UnoGameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
