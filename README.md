# Shelf
*Shelf* is a command line tool which enables users to bookmark frequently used directories. A book represents a unique directory and it is associated with a category and a tag.

## Usage
```bash
# Store a book of cwd
shelf store <category> <tag>
# Store a book of [path]
shelf store <category> <tag> [path]

# Show all books
shelf visit
# Show all books associated with <category>
shelf visit <category>

# Show a single book
shelf read <category> <tag>

# Remove books in <category>
shelf throw <category>
# Remove a single book
shelf throw <category> [tag]
```

> Disclaimer: This is developed for my personal use. It might be useful for you or not at all.
