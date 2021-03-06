use std::collections::HashMap;
use std::boxed::Box;
#[derive( PartialEq, Debug)]
pub enum RustValue {
    F(f64),
    B(bool),
    S(String),
    U,
    N,
    O(HashMap<String,RustValue>)
}

pub trait Thing<'p> {
    fn getItem(&mut self, key:&mut String ) -> &Thing<'p>;
    fn keys(&self) -> RustValue;
    fn setItem(&mut self,key:String, value:&'p Thing<'p>);
    fn value(&self) -> RustValue;
}

pub trait Evaluable {
    fn eval(&self,context: &mut Thing) -> Thing;
}
use std::marker::PhantomData;

pub struct Undef<'p>{
    phantom: PhantomData<&'p i64>,
}

impl<'p> Thing<'p> for Undef<'p>{
    fn getItem(&mut self, key:&mut String ) -> &Thing<'p>{
        return &Undef {phantom:PhantomData}
    }
    fn keys(&self) -> RustValue{
        let mut keyList:HashMap<String,RustValue>=HashMap::new();
        let mut i=0;
        keyList.insert("length".to_string(),RustValue::F(i as f64));
        return RustValue::O(keyList);
    }
    fn setItem(&mut self,key: String, value:&Thing){
        return
    }
    fn value(&self) -> RustValue{
        return RustValue::U;
    }
}
impl<'p> Undef<'p>{
    fn new<'q>() -> Undef<'q>{
        Undef {phantom:PhantomData}
    }
}

pub struct Object<'c, 's: 'c> {
    map:HashMap<String,&'c Thing<'c>>,
    n:&'s i64
}

impl<'p,'q> Thing<'p> for Object<'p,'q>{
    fn getItem(&mut self, key:&mut String) -> &Thing<'p>{
        return match (self).map.get(key) {
            Some(t) => {
                *t
            },
            None => {
                &Undef {phantom:PhantomData}
            }
        }
    }
    fn keys(&self) -> RustValue{
        let mut keyList:HashMap<String,RustValue>=HashMap::new();
        let mut i=0;
        for key in self.map.keys(){
            keyList.insert(i.to_string(),RustValue::S(key.to_string()));
            i=i+1;
        }
        keyList.insert("length".to_owned(),RustValue::F(i as f64));
        return RustValue::O(keyList);
    }
    fn setItem(&mut self,key: String, value:&'p Thing<'p>){
        self.map.insert(key,value);
    }
    fn value(&self) -> RustValue{
        let mut converted:HashMap<String,RustValue>=HashMap::new();
        for (k,v) in &self.map{
            converted.insert(k.to_string(),v.value());
        }
        return RustValue::O(converted);
    }
}

impl<'p,'q> Object<'p,'q>{
    pub fn new<'g>(map:HashMap<String,&'g Thing<'g>>) -> Object<'g,'g>{
        Object {map:map,n:&10}
    }
    pub fn empty<'g>() -> Object<'g,'g>{
        Object {map:HashMap::new(),n:&0}
    }
}

pub struct Number {
    v:f64
}
trait Small: Sized { }
impl Small for Number{}

impl<'p> Thing<'p> for Number{
    fn getItem(&mut self, key:&mut String) -> &Thing<'p>{
        return self;
        /*let m=Undef::new();
        return &'p m;*/
    }
    fn keys(&self) -> RustValue{
        let mut keyList:HashMap<String,RustValue>=HashMap::new();
        let mut i=0;
        keyList.insert("length".to_owned(),RustValue::F(i as f64));
        return RustValue::O(keyList);
    }
    fn setItem(&mut self,key:String, value:&Thing){
        return
    }
    fn value(&self) -> RustValue{
        return RustValue::F(self.v);
    }
}

impl Number{
    pub fn new(n:f64) -> Number{
        Number {v:n}
    }
}

pub struct Boolean {
    v:bool
}
impl Small for Boolean{}

impl<'p> Thing<'p> for Boolean{
    fn getItem(&mut self, key:&mut String) -> &Thing<'p>{
        return self;
        /*let m=Undef::new();
        return &'p m;*/
    }
    fn keys(&self) -> RustValue{
        let mut keyList:HashMap<String,RustValue>=HashMap::new();
        let mut i=0;
        keyList.insert("length".to_owned(),RustValue::F(i as f64));
        return RustValue::O(keyList);
    }
    fn setItem(&mut self,key:String, value:&Thing){
        return
    }
    fn value(&self) -> RustValue{
        return RustValue::B(self.v);
    }
}

impl Boolean{
    pub fn new(n:bool) -> Boolean{
        Boolean {v:n}
    }
}
