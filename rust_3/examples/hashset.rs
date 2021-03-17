// Once again, HashSet is not included in the prelude
use std::collections::HashSet;

let mut books = HashSet::new();

// Add some books.
books.insert("The Making of the Indebted Man".to_string());
books.insert("Introduction to Civil War".to_string());

if !books.contains("The Concept of the Political") {}

// Remove an item
books.remove("Time, Labor, and Social Domination");
