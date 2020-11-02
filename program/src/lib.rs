pub mod entrypoint;
pub mod error;
pub mod processor;
pub mod instructions;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
