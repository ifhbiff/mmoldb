use mmolb_parsing::enums::{Place, Slot, SlotDiscriminants};
use mmolb_parsing::parsed_event::PlacedPlayer;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum BestEffortSlot {
    Slot(Slot),
    SlotType(SlotDiscriminants),
    GenericPitcher,
}

impl Display for BestEffortSlot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BestEffortSlot::Slot(s) => write!(f, "{}", s),
            // This might not be right...
            BestEffortSlot::SlotType(st) => write!(f, "{}", st),
            BestEffortSlot::GenericPitcher => write!(f, "P"),
        }
    }
}

impl From<Place> for BestEffortSlot {
    fn from(value: Place) -> Self {
        match value {
            Place::Pitcher => BestEffortSlot::GenericPitcher,
            Place::Catcher => BestEffortSlot::Slot(Slot::Catcher),
            Place::FirstBaseman => BestEffortSlot::Slot(Slot::FirstBaseman),
            Place::SecondBaseman => BestEffortSlot::Slot(Slot::SecondBaseman),
            Place::ThirdBaseman => BestEffortSlot::Slot(Slot::ThirdBaseman),
            Place::ShortStop => BestEffortSlot::Slot(Slot::ShortStop),
            Place::LeftField => BestEffortSlot::Slot(Slot::LeftField),
            Place::CenterField => BestEffortSlot::Slot(Slot::CenterField),
            Place::RightField => BestEffortSlot::Slot(Slot::RightField),
            Place::StartingPitcher(Some(i)) => BestEffortSlot::Slot(Slot::StartingPitcher(i)),
            Place::ReliefPitcher(Some(i)) => BestEffortSlot::Slot(Slot::ReliefPitcher(i)),
            Place::Closer => BestEffortSlot::Slot(Slot::Closer),
            Place::DesignatedHitter => BestEffortSlot::Slot(Slot::DesignatedHitter),
            Place::StartingPitcher(None) => {
                BestEffortSlot::SlotType(SlotDiscriminants::StartingPitcher)
            }
            Place::ReliefPitcher(None) => {
                BestEffortSlot::SlotType(SlotDiscriminants::ReliefPitcher)
            }
        }
    }
}

impl FromStr for BestEffortSlot {
    type Err = <Slot as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let slot = s.parse::<Slot>()?;
        Ok(Self::Slot(slot))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BestEffortSlottedPlayer<StrT> {
    pub name: StrT,
    pub slot: BestEffortSlot,
}

impl<StrT> From<PlacedPlayer<StrT>> for BestEffortSlottedPlayer<StrT> {
    fn from(value: PlacedPlayer<StrT>) -> Self {
        BestEffortSlottedPlayer {
            name: value.name,
            slot: value.place.into(),
        }
    }
}

impl<StrT: Display> Display for BestEffortSlottedPlayer<StrT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.slot, self.name)
    }
}
