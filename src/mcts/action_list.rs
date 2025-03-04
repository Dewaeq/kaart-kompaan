pub trait ActionList<T> {
    fn uninit() -> Self;

    fn pop_random(&mut self) -> Option<T>;

    fn push(&mut self, action: T);

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn has(&self, item: &T) -> bool
    where
        T: PartialEq;

    fn without(&self, other: &Self) -> Self;
}

//impl<T> ActionList<T> for Vec<T> {
//    fn pop_random(&mut self) -> Option<T> {
//        self.pop()
//    }
//
//    fn is_empty(&self) -> bool {
//        self.is_empty()
//    }
//
//    fn has(&self, item: &T) -> bool
//    where
//        T: PartialEq,
//    {
//        self.contains(item)
//    }
//}
