use command::parser::comma_list_or;

error_chain! {
    errors {
        PlayerCount(min: usize, max: usize, given: usize) {
            description("incorrect player count")
            display(
                "not for {} players, expected {}",
                given,
                player_range_output(*min, *max),
            )
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
            display("internal error: {}", message)
        }
        Parse(message: Option<String>, expected: Vec<String>, offset: usize) {
            description("parse error")
            display("{}expected {}", message.as_ref().map(|m| format!("{}, ", m))
                .unwrap_or_else(|| "".to_string()), comma_list_or(expected))
        }
    }
}

fn player_range_output(min: usize, max: usize) -> String {
    if min == max {
        format!("{}", min)
    } else {
        format!("{} to {}", min, max)
    }
}
