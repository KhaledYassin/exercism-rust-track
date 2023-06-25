use std::{collections::HashMap, ops::{Index, IndexMut}};

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(usize);
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]

/// `CallbackId` is a tuple of the index of the cell in the reactor and the index of the callback within that cell.
pub struct CallbackId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

impl CellId {
    fn get_id(&self) -> usize {
        match self {
            CellId::Input(id) => id.0,
            CellId::Compute(id) => id.0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct Callback<'a, T> {
    id: CallbackId,
    function: Box<dyn 'a + FnMut(T)>,
}

pub struct Cell<'a, T> {
    id: CellId,
    value: T,
    compute_function: Option<Box<dyn 'a + Fn(&[T]) -> T>>,
    callbacks: HashMap<usize, Callback<'a, T>>,
    callback_id_counter: usize,
    depedencies: Vec<CellId>,
}

impl<'a, T: Copy + PartialEq> Cell<'a, T> {
    fn new(id: CellId, value: T) -> Self {
        Self {
            id,
            value,
            compute_function: None,
            callbacks: HashMap::new(),
            callback_id_counter: 0,
            depedencies: Vec::new(),
        }
    }

    fn new_with_compute(id: CellId, value: T, compute_function: Box<dyn Fn(&[T]) -> T>) -> Self {
        Self {
            id,
            value,
            compute_function: Some(compute_function),
            callbacks: HashMap::new(),
            callback_id_counter: 0,
            depedencies: Vec::new(),
        }
    }

    fn set_value(&mut self, new_value: T) {
        self.value = new_value;
    }

    fn add_callback(&mut self, callback: Callback<'a, T>) {
        self.callbacks.insert(callback.id.0, callback);
    }

    fn remove_callback(&mut self, callback_id: usize) -> Option<Callback<T>> {
        self.callbacks.remove(&callback_id)
    }
}

pub struct Reactor<'a, T> {
    cells: Vec<Cell<'a, T>>,
    depedents_map: HashMap<CellId, Vec<CellId>>,
}

impl<'a, T: Copy + PartialEq> Default for Reactor<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T: Copy + PartialEq> Index<CellId> for Reactor<'a, T> {
    type Output = Cell<'a, T>;

    fn index(&self, index: CellId) -> &Self::Output {
        &self.cells[index.get_id()]
    }
}

impl<'a, T: Copy + PartialEq> IndexMut<CellId> for Reactor<'a, T> {
    fn index_mut(&mut self, index: CellId) -> &mut Self::Output {
        &mut self.cells[index.get_id()]
    }
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            depedents_map: HashMap::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, value: T) -> InputCellId {
        let id = self.cells.len();

        let input_id = InputCellId(id);

        let cell = Cell::new(CellId::Input(input_id), value);

        self.cells.push(cell);

        input_id
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: 'a + Fn(&[T]) -> T + 'static>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        let values_result = self.get_values(dependencies);

        match values_result {
            Ok(values) => {
                let cells = &mut self.cells;

                let value = compute_func(&values);

                let id = cells.len();

                let compute_id = ComputeCellId(id);

                let cell_id = CellId::Compute(compute_id);

                let mut cell = Cell::new_with_compute(cell_id, value, Box::new(compute_func));

                for dependency in dependencies {
                    let dependents = self.depedents_map.entry(*dependency).or_default();

                    dependents.push(cell_id);
                    cell.depedencies.push(*dependency);
                }

                cells.push(cell);

                Ok(compute_id)
            },
            Err(cell_id) => Err(cell_id),
        }
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        self.cells.get(id.get_id()).map(|c| c.value)
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        match self.cells.get_mut(id.0) {
            Some(cell) => {
                cell.set_value(new_value);

                let mut callback_cells = HashMap::new();

                self.update_dependents(&CellId::Input(id), &mut callback_cells);

                for (callback_cell_id, old_value) in callback_cells {
                    let callback_cell = &mut self[callback_cell_id];

                    if callback_cell.value != old_value {
                        for (_, callback) in callback_cell.callbacks.iter_mut() {
                            (callback.function)(callback_cell.value);
                        }
                    }
                }

                true
            }
            None => false,
        }
    }

    fn update_dependents(&mut self, cell_id: &CellId, callback_cells: &mut HashMap<CellId, T>) {
        if let Some(dependents) = self.depedents_map.clone().get(cell_id) {
            for dependent in dependents {
                let cell = &self[*dependent];
                let computed_cell_id = cell.id;
                let values_result = self.get_values(&cell.depedencies);

                let new_value = cell.compute_function.as_ref().unwrap()(&values_result.unwrap());

                if new_value != cell.value {
                    callback_cells.entry(computed_cell_id).or_insert(cell.value);
                    self[computed_cell_id].value = new_value;
                    self.update_dependents(&computed_cell_id, callback_cells);
                }
            }
        }
    }

    fn get_values(&self, cell_ids: &[CellId]) -> Result<Vec<T>, CellId>  {
        let mut values = vec![];
        for id in cell_ids {
            if let Some(value) = self.value(*id) {
                values.push(value)
            } else {
                return Err(*id)
            }
        }
        Ok(values)
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: 'a + FnMut(T)>(
        &mut self,
        id: ComputeCellId,
        callback_function: F,
    ) -> Option<CallbackId> {
        match self.cells.get_mut(id.0) {
            Some(cell) => {
                let callback_index = cell.callback_id_counter;
                cell.callback_id_counter += 1;

                let callback_id = CallbackId(callback_index);

                let callback = Callback {
                    id: callback_id,
                    function: Box::new(callback_function),
                };

                cell.add_callback(callback);

                Some(callback_id)
            }
            None => None,
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell_id: ComputeCellId,
        callback_id: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        match self.cells.get_mut(cell_id.0) {
            Some(cell) => {
                let callbacks = &cell.callbacks;

                match callbacks
                    .iter()
                    .find(|(_, callback)| callback.id == callback_id)
                {
                    Some((callback_index, _)) => {
                        cell.remove_callback(*callback_index);
                        Ok(())
                    }
                    None => Err(RemoveCallbackError::NonexistentCallback),
                }
            }
            None => Err(RemoveCallbackError::NonexistentCell),
        }
    }
}
