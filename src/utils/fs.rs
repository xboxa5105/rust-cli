use mockall::automock;
use std::fs;
use std::io;

#[automock]
pub trait FileReader {
    fn read_to_string(&self, path: &str) -> io::Result<String>;
    fn write(&self, path: &str, content: &str) -> io::Result<()>;
}

pub struct RealFileReader;

impl FileReader for RealFileReader {
    fn read_to_string(&self, path: &str) -> io::Result<String> {
        fs::read_to_string(path)
    }

    fn write(&self, path: &str, content: &str) -> io::Result<()> {
        fs::write(content, path)
    }
}

mod test {
    use super::*;
    use mockall::predicate::eq;
    use std::io::ErrorKind;

    #[test]
    fn test_read_to_string() {
        let mut mock = MockFileReader::new();
        mock.expect_read_to_string()
            .with(eq("test.txt"))
            .returning(|_| Ok("test".to_string()));
        assert_eq!(mock.read_to_string("test.txt").unwrap(), "test");
    }

    #[test]
    fn test_read_to_string_error() {
        let mut mock = MockFileReader::new();
        mock.expect_read_to_string()
            .with(eq("test.txt"))
            .returning(|_| Err(io::Error::new(ErrorKind::Other, "test")));
        assert_eq!(mock.read_to_string("test.txt").unwrap_err().kind(), ErrorKind::Other);
    }

    #[test]
    fn test_write() {
        let mut mock = MockFileReader::new();
        mock.expect_write()
            .with(eq("test.txt"), eq("test"))
            .returning(|_, _| Ok(()));
        assert_eq!(mock.write("test.txt", "test").unwrap(), ());
    }

    #[test]
    fn test_write_error() {
        let mut mock = MockFileReader::new();
        mock.expect_write()
            .with(eq("test.txt"), eq("test"))
            .returning(|_, _| Err(io::Error::new(ErrorKind::Other, "test")));
        assert_eq!(mock.write("test.txt", "test").unwrap_err().kind(), ErrorKind::Other);
    }
}
