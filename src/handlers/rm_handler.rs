pub struct RmHandler;

impl<'a> CommandHandler<'a> for RmHandler {
    fn handle<T>(self: &'a Self, command:&T)->Result<(), &str> where T:  {
        todo!()
    }
}