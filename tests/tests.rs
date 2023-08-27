#[cfg(test)]
mod tests {
    use http_error_derive::HttpError;

    #[derive(HttpError)]
    enum Test {
        #[http(code = 400, message = "You fucked up somewhere, lol")]
        ErrorWithEverything,
        #[http(code = 200)]
        ErrorWithCode,
        #[http(message = "Hello my friend, what are you doing here")]
        ErrorWithMessage,
        ErrorWithNothing,
    }

    #[test]
    fn error_with_everything() {
        assert_eq!(Test::ErrorWithEverything.http_code(), Some(400));
        assert_eq!(
            Test::ErrorWithEverything.http_message(),
            Some("You fucked up somewhere, lol")
        );
    }

    #[test]
    fn error_with_code() {
        assert_eq!(Test::ErrorWithCode.http_code(), Some(200));
        assert_eq!(Test::ErrorWithCode.http_message(), None);
    }

    #[test]
    fn error_with_message() {
        assert_eq!(Test::ErrorWithMessage.http_code(), None);
        assert_eq!(
            Test::ErrorWithMessage.http_message(),
            Some("Hello my friend, what are you doing here")
        );
    }

    #[test]
    fn error_with_nothing() {
        assert_eq!(Test::ErrorWithNothing.http_code(), None);
        assert_eq!(Test::ErrorWithNothing.http_message(), None);
    }
}
