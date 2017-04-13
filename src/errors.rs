error_chain! {
    errors {
        PlayerCount(min: usize, max: usize, given: usize) {
            description("incorrect player count")
            display("not for {} players, expecting {} to {}", given, min, max)
        }
        InvalidInput(message: String) {
            description("invalid input")
            display("{}", message)
        }
        NotYourTurn {
            description("not your turn")
        }
        Finished {
            description("game is already finished")
        }
        Internal(message: String) {
            description("internal error")
            display("{}", message)
        }
    }
}
