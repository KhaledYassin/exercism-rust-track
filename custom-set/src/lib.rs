#[derive(Debug)]
pub struct CustomSet<T> {
    unique_list: Vec<T>,
}

impl<T> PartialEq for CustomSet<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.unique_list.len() == other.unique_list.len() {
            return self
                .unique_list
                .iter()
                .all(|elem| other.unique_list.contains(elem));
        }
        false
    }
}

impl<T: Clone + Copy> CustomSet<T>
where
    T: PartialEq,
{
    pub fn new(input: &[T]) -> Self {
        Self {
            unique_list: input.to_vec(),
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.unique_list.iter().any(|item| item == element)
    }

    pub fn add(&mut self, element: T) {
        if !self.unique_list.contains(&element) {
            self.unique_list.push(element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.unique_list.iter().all(|item| other.contains(item))
    }

    pub fn is_empty(&self) -> bool {
        self.unique_list.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.unique_list.iter().all(|item| !other.contains(item))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let intersection = self
            .unique_list
            .iter()
            .filter(|item| other.contains(item))
            .copied()
            .collect::<Vec<_>>();

        Self {
            unique_list: intersection,
        }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let difference = self
            .unique_list
            .iter()
            .filter(|item| !other.contains(item))
            .copied()
            .collect::<Vec<_>>();

        Self {
            unique_list: difference,
        }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut union_set = Self::new(&[]);

        for item in self.unique_list.iter().chain(other.unique_list.iter()) {
            if !union_set.contains(item) {
                union_set.add(*item)
            }
        }

        union_set
    }
}
