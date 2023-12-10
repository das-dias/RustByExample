use gui::{Component, Window, Button};
struct SelectBox {
  pos:      (u32, u32),
  width:    u32,
  height:   u32,
  options:  Vec<String>,
}
impl Component for SelectBox {
  fn draw(&self) {
    println!("Drawn SelectBox!");
    for option in self.options.iter() {
      println!("{}", option)
    }
  }
  fn bounding_box(&self) -> ((u32, u32), (u32, u32)) {
    return (
      self.pos, 
      (self.pos.0 + self.width, self.pos.1 + self.height)
    )
  }
}
fn main() {
  let win = Window {
    components: vec![
      Box::new(SelectBox {
        pos: (40, 40),
        width: 75,
        height: 10,
        options: vec![
          String::from("Yes"),
          String::from("Maybe"),
          String::from("No"),
        ],
      }),
      Box::new(Button {
        pos: (20, 20),
        width: 50,
        height: 10,
        clicked: false,
        label: String::from("OK"),
      }),
    ],
  };
  win.run();

  /* blog post implementation using state-patterns */
  let mut post = blog::Post::new();
  post.add_text("I ate state salad for lunch today");
  assert_eq!("", post.content());  
  post.request_review();
  assert_eq!("", post.content());  
  post.approve();
  assert_eq!("I ate state salad for lunch today", post.content());
  println!("{}",post.content());

  /* blog post implementation using enum-patterns */
  let mut post = blog2::Post::new();
  post.add_text("I ate enum salad for lunch today");
  assert_eq!("", post.content());  
  post.request_review();
  assert_eq!("", post.content());  
  post.approve();
  assert_eq!("I ate enum salad for lunch today", post.content());
  println!("{}",post.content());

  /* blog post implementation using state-as-types-patterns */
  let mut post_draft = blog3::Post::new();
  post_draft.add_text("I ate type salad for lunch today");
  let pending_review_post = post_draft.request_review();
  let post = pending_review_post.approve();
  assert_eq!("I ate type salad for lunch today", post.content());
  println!("{}",post.content())
}

mod blog3 {
  //! blog mod implementation using State types
  //! each state transition is signaled by changing 
  //! the type of the state variable
  pub struct Post {
    _content: String,
  }
  impl Post {
    pub fn new() -> PostDraft {
      PostDraft {
        _content: String::new()
      }
    }
    pub fn content(&self) -> &str {
        &self._content
    }
  }
  pub struct PostDraft {
    _content: String
  }
  impl PostDraft {
    pub fn add_text(&mut self, text: &str) {
      self._content.push_str(text)
    }
    pub fn request_review(self) -> PendingReview {
      PendingReview{_content: self._content }
    }
  }
  /* calling "add_text" on a PendingReview will 
  result in a compiler error becasue the struct doesn't 
  even implement the method. This is an 
  alternative way of using composition. */
  pub struct PendingReview {
    _content: String
  }
  impl PendingReview {
    pub fn approve(self) -> Post {
      Post {_content: self._content}
    }
  }
}

mod blog2 {
  //! blog mod implementation using State enums
  //! Using enums makes the implementation easier, 
  //! or more straightfoward, but scaling this solution
  //! might be more difficult that scaling the state pattern
  pub struct Post {
    _state: State,
    _content: String,
  }
  impl Post {
    pub fn new() -> Self {
      Post {
        _state: State::Draft(String::new()),
        _content: String::new()
      }
    }
    pub fn add_text(&mut self, text: &str) {
      match self._state {
        State::Draft(_) => self._content.push_str(text),
        _ => ()
      }
    }
    pub fn request_review(&mut self) {
      match self._state {
        State::Draft(_) => self._state = State::PendingReview(String::new()),
        _ => () 
      }
    }
    pub fn approve(&mut self) {
      match self._state {
        State::PendingReview(_) => self._state = State::Published(self._content.clone()),
        _ => ()
      }
    }
    pub fn content(&self) -> &str {
      match &self._state {
        State::Published(content) => content.as_str(),
        _ => ""
      }
    }
  }

  enum State {
    Draft(String),
    PendingReview(String),
    Published(String)
  }
}

mod blog {
  //! blog mod implementation using State trait
  pub struct Post {
    _state: Option<Box<dyn State>>,
    _content: String,
  }
  /* advantages of the state pattern: the request_review method 
  on Post is the same no matter its state value. 
  Each state is responsible for its own rules. */
  impl Post {
    pub fn new() -> Self {
      Post {
        _state: Some(Box::new(Draft {})),
        _content: String::new()
      }
    }
    pub fn add_text(&mut self, text: &str) {
      self._content.push_str(text)
    }
    pub fn request_review(&mut self) {
      /* We need to set state to None temporarily rather 
      than setting it directly with code like 
      self._state = self._state.request_review(); 
      to get ownership of the state value. This ensures Post can’t 
      use the old state value after we’ve transformed it 
      into a new state. */
      if let Some(state) = self._state.take() {
        self._state = Some(state.request_review())
      }
    }
    pub fn approve(&mut self) {
      if let Some(state) = self._state.take() {
        self._state = Some(state.approve())
      }
    }
    pub fn content(&self) -> &str {
      /* as_ref() gives Option<Box<State>>, unwrap() gives Box<State> */
      self._state.as_ref().unwrap().content(self)
    }
  }
  
  trait State {
    /* self: Box<Self> : This syntax means the method is only 
    valid when called on a Box holding the type. */
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
  }

  struct Draft {}
  impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
      Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
      self /* return self to keep in the Draft state!
            No posts can be approved before review_request 
            has been called upon them. */
    }
  }

  struct PendingReview {}
  impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
      self /* keep in review process */
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
      Box::new(Published {})
    }
  }

  struct Published {}
  impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {self}
    fn approve(self: Box<Self>) -> Box<dyn State> {self}
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post._content
    }
  }
  
}

mod gui {
  /* Composition using traits */
  
  /* This trait object enables 
   the Window object to hold different
   object-like types in its components vector,
   as long as this types implement the 
   Component trait. This would not be possible
   if Window was defined using a generic type <T> 
   instead.
   */
  pub trait Component {
    fn draw(&self);
    fn bounding_box(&self) -> ((u32, u32), (u32, u32));
  }    
  pub struct Window {
    pub components: Vec<Box<dyn Component>> // a vector of Exclusive-Reference 
  }                                     // Smart Points to any object implementing 
                                        // the Component Trait 
  impl Window {
    pub fn run(&self) {
      println!("Hello Window!");
      for component in self.components.iter()  {
        component.draw();
      }
    }
  }
  
  pub struct Button {
    pub pos:     (u32, u32),
    pub width:   u32,
    pub height:  u32,
    pub label:   String,
    pub clicked: bool
  }
  impl Component for Button{
    fn draw(&self){
        println!("Button drawn!")
    }
    fn bounding_box(&self) -> ((u32, u32), (u32,u32)) {
        return (
            self.pos, 
            (self.pos.0 + self.width, self.pos.1 + self.height)
        )
    }
  }
    
}