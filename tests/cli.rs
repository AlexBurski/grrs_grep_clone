use grrs::{match_line, search_sequential, search_parallel};
   use std::io::Cursor;

   #[test]
   fn test_match_line() {
       assert!(match_line("hello world", "world", false));
       assert!(!match_line("hello world", "World", false));
       assert!(match_line("hello world", "World", true));
       assert!(!match_line("hello world", "universe", false));
   }

   #[test]
   fn test_search_sequential() {
       let content = "Hello\nWorld\nTest\nWorld\n";
       let reader = Cursor::new(content);
       let mut output = Vec::new();
       search_sequential(reader, "World", false, |line| output.push(line.to_string())).unwrap();
       assert_eq!(output, vec!["World", "World"]);
   }

   #[test]
   fn test_search_parallel() {
       let content = "Hello\nWorld\nTest\nWorld\n";
       let reader = Cursor::new(content);
       let output = std::sync::Mutex::new(Vec::new());
       search_parallel(reader, "World", false, |line| output.lock().unwrap().push(line.to_string())).unwrap();
       let output = output.into_inner().unwrap();
       assert_eq!(output.len(), 2);
       assert!(output.contains(&"World".to_string()));
   }
   