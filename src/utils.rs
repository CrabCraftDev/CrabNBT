
// TODO: Is it really a good idea to introduce two generic traits in an NBT library?

pub trait TryAsRef<T: ?Sized> {
    fn try_as_ref(&self) -> Option<&T>;
}
pub trait TryAsMut<T: ?Sized> {
    fn try_as_mut(&mut self) -> Option<&mut T>;
}