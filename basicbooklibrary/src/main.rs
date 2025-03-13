use std::fmt::Display;
use std::fmt;
struct Book{
    title:String,
    author:String,
    year_published:u32,
    checked_out_by:Option<String>,
}

struct Library{
    books: Vec<Book>,
}

impl Book{
    fn new (title:String, author:String, year_published:u32) -> Book{
        Book{
            title,
            author,
            year_published,
            checked_out_by:None,
        }
    }
    fn check_out(&mut self, name:String){
        self.checked_out_by = Some(name);
    }
    fn is_checked_out(&self)-> bool{
        self.checked_out_by.is_some()
    }
    fn checked_out_by(&self)-> Option<String>{
        self.checked_out_by.clone()
    }
}
impl Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       let borrowed_by= match self.checked_out_by {
            Some(ref s) => {s},
            None => {"not borrowed"}
        };
        write!(f, "({}, {},{},{})", self.title, self.author, self.year_published, borrowed_by)
    }
}

impl Library{
    fn add_book(&mut self,  book: Book){
        self.books.push(book);
    }
    fn list_books(&self) -> &Vec<Book>{
        &self.books
    }
    fn find_book_by_title(&mut self, title:&str) -> Option<&mut Book>{
        // for book in &self.books{
        //     if book.title == title {
        //         return Some(book);
        //     }
        // }
        // None
        //Or we can use an itr
        self.books.iter_mut().find(|book| book.title == title)//which will return an optional, or null

    }
    fn delete_book_by_title(&mut self, title:&str){
        self.books.retain(|book| book.title != title);
    }
    fn print_books(&self){
        for book in &self.books{
            println!("{}", book);
        }
    }
    fn borrow_book(&mut self, title:&str,borrower_name:&str ){
        let book_to_borrow= self.find_book_by_title(title);
        if let Some(book_to_borrow) = book_to_borrow {
            book_to_borrow.check_out(borrower_name.to_string());

        }else {
            println!("{} not found in library", title);
        }

    }
}


fn main() {

    let book1= Book::new("title1".to_string(),"author1".to_string(),2002);
    let book2= Book::new("title2".to_string(),"author2".to_string(),2003);
    let book3= Book::new("title3".to_string(),"author3".to_string(),2004);
    let book4= Book::new("title4".to_string(),"author4".to_string(),2005);

    let mut library=Library{books: vec![book1,book2,book3,book4]};

    let book5=Book::new("title5".to_string(),"author5".to_string(),2006);
    library.add_book(book5);

    library.print_books();
    println!("----------------");
    library.borrow_book("title3","ahmad");

    library.print_books();
    println!("----------------");

    library.delete_book_by_title("title3");
    library.print_books();
    library.borrow_book("title3","ahmad");


}
