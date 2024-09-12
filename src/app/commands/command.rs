pub trait Command {
    type Input;
    type Output;
    fn execute(self, input: Self::Input) -> Self::Output;
}


pub trait AsyncCommand {
    type Input;
    type Output;
    async fn execute(self, input: Self::Input) -> Self::Output;
}