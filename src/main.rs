use rand::Rng;

pub trait State {
    type Input;
    type Output;
    type Error;

    fn validate(&self, input: &Self::Input) -> Option<Self::Error>;

    fn compute(&self, input: &Self::Input) -> Option<Self::Output>;

    fn transition(&mut self, input: &Self::Input);

    fn next(
        &mut self,
        transition_event: &Self::Input,
    ) -> Result<Option<Self::Output>, Self::Error> {
        let res = self.validate(transition_event);
        if let Some(x) = res {
            Err(x)
        } else {
            let out = self.compute(transition_event);
            self.transition(transition_event);
            Ok(out)
        }
    }
}

//a player id type, here usize for demo
type PlayerId = usize;

#[derive(Debug, PartialEq, Clone, Copy)]
enum CardType {
    Red,
    Blue,
    Green,
    Yellow,
    // if one wants to implement special cards such as draw 4s and wild,
    // one would put it here
}

//every card is represented by a struct containing CardType(just the colour for now)
//and the number,
//if you want to include reverse, skip, etc
//you would need to represent another enum for the number
#[derive(Debug, PartialEq, Clone)]
struct UnoCard {
    card_type: CardType,
    number: usize,
}

//every uno event has an associated player id, i.e the player who sent that event
//the event also has an event type
struct UnoEvent {
    id: PlayerId,
    event_type: UnoEventType,
}

//the event type which describes what sort of event it is
//the input type for this state
#[derive(PartialEq)]
enum UnoEventType {
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
enum UnoError {
    //used when the discarded card is not of the same colour or the same number
    IncorrectCard,
    //used when the expected player did not send the event
    IncorrectPlayer,
    UnexpectedEvent,
}

#[derive(Debug, PartialEq, Eq)]
enum UnoStateType {
    WaitingForDiscard,
    WaitingForDraw,
}

//the output type
#[derive(Debug)]
enum UnoOutput {
    //sent after a draw event with the drawn card
    Card(UnoCard),
}

//the state type
#[derive(Debug)]
struct UnoGameState {
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

    fn validate(&self, t: &Self::Input) -> Option<UnoError> {
        if self.expected_player_turn != t.id {
            return Some(UnoError::IncorrectPlayer);
        }
        match (&self.state_type, &t.event_type) {
            (UnoStateType::WaitingForDiscard, UnoEventType::Discard(c)) => {
                if self.top_card.card_type == c.card_type || self.top_card.number == c.number {
                    None
                } else {
                    Some(UnoError::IncorrectCard)
                }
            }
            (UnoStateType::WaitingForDiscard, UnoEventType::NoCard) => None,
            (UnoStateType::WaitingForDraw, UnoEventType::Draw) => None,
            _ => Some(UnoError::UnexpectedEvent),
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

fn main() {
    let mut state = UnoGameState {
        players_num: 4,
        top_card: UnoCard {
            card_type: CardType::Red,
            number: 4,
        },
        state_type: UnoStateType::WaitingForDiscard,
        expected_player_turn: 0,
    };

    println!("{:?}", state);
    state
        .next(&UnoEvent {
            id: 0,
            event_type: UnoEventType::Discard(UnoCard {
                card_type: CardType::Blue,
                number: 4,
            }),
        })
        .expect("Error");

    println!("{:?}", state);

    state
        .next(&UnoEvent {
            id: 1,
            event_type: UnoEventType::NoCard,
        })
        .expect("Error");

    println!("{:?}", state);

    let err = state
        .next(&UnoEvent {
            id: 1,
            event_type: UnoEventType::NoCard,
        })
        .expect_err("No Error");

    println!("{:?}", err);

    let out = state
        .next(&UnoEvent {
            id: 1,
            event_type: UnoEventType::Draw,
        })
        .expect("Error")
        .expect("No O/P");
    println!("{:?}", out);

    println!("{:?}", state);

    let err = state
        .next(&UnoEvent {
            id: 3,
            event_type: UnoEventType::Draw,
        })
        .expect_err("No Error");

    println!("{:?}", err);

    println!("{:?}", state);

    let err = state
        .next(&UnoEvent {
            id: 1,
            event_type: UnoEventType::Discard(UnoCard {
                card_type: CardType::Green,
                number: 5,
            }),
        })
        .expect_err("No Error");

    println!("{:?}", err);

    println!("{:?}", state);
}
