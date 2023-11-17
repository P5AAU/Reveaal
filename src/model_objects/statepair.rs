use edbm::zones::OwnedFederation;

use crate::transition_systems::{LocationTree, TransitionSystemPtr};
use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
};

#[derive(Clone, Debug)]
pub struct StatePair {
    pub locations1: RefCell<LocationTree>,
    pub locations2: RefCell<LocationTree>,
    /// The sentinel (Option) allows us to take ownership of the internal fed from a mutable reference
    zone_sentinel: RefCell<Option<OwnedFederation>>,
}

impl StatePair {
    pub fn from_locations(
        dimensions: usize,
        locations1: LocationTree,
        locations2: LocationTree,
    ) -> StatePair {
        let zone = OwnedFederation::init(dimensions);

        StatePair {
            locations1: RefCell::new(locations1),
            locations2: RefCell::new(locations2),
            zone_sentinel: RefCell::new(Some(zone)),
        }
    }

    pub fn get_locations1(&self) -> &LocationTree {
        &self.locations1.borrow()
    }

    pub fn get_locations2(&self) -> &LocationTree {
        &self.locations2.borrow()
    }

    //Used to allow borrowing both states as mutable
    pub fn get_mut_states(&self, is_states1: bool) -> (&mut LocationTree, &mut LocationTree) {
        if is_states1 {
            (
                &mut self.locations1.borrow_mut(),
                &mut self.locations2.borrow_mut(),
            )
        } else {
            (
                &mut self.locations2.borrow_mut(),
                &mut self.locations1.borrow_mut(),
            )
        }
    }

    pub fn get_locations(&self, is_states1: bool) -> (&LocationTree, &LocationTree) {
        if is_states1 {
            (&self.locations1.borrow(), &self.locations2.borrow())
        } else {
            (&self.locations2.borrow(), &self.locations1.borrow())
        }
    }

    pub fn clone_zone(&self) -> OwnedFederation {
        self.ref_zone().clone()
    }

    pub fn ref_zone(&self) -> &OwnedFederation {
        let borrowed_sentinel = self.zone_sentinel.borrow();
        borrowed_sentinel.as_ref().unwrap()
    }

    pub fn take_zone(&self) -> OwnedFederation {
        self.zone_sentinel.borrow_mut().take().unwrap()
    }

    pub fn set_zone(&self, zone: OwnedFederation) {
        *self.zone_sentinel.borrow_mut() = Some(zone);
    }

    pub fn extrapolate_max_bounds(&self, sys1: &TransitionSystemPtr, sys2: &TransitionSystemPtr) {
        let mut bounds = sys1.get_local_max_bounds(&self.locations1.borrow());
        bounds.add_bounds(&sys2.get_local_max_bounds(&self.locations2.borrow()));
        let zone = self.take_zone().extrapolate_max_bounds(&bounds);
        self.set_zone(zone);
    }
}

impl Display for StatePair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Pair: 1:{} where {}, 2:{} where {}, zone: {}",
            self.locations1.borrow().id,
            self.locations1
                .borrow()
                .get_invariants()
                .map(|f| f.to_string())
                .unwrap_or_else(|| "no invariant".to_string()),
            self.locations2.borrow().id,
            self.locations2
                .borrow()
                .get_invariants()
                .map(|f| f.to_string())
                .unwrap_or_else(|| "no invariant".to_string()),
            self.ref_zone()
        ))?;

        Ok(())
    }
}
