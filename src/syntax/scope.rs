use std::collections::HashMap;
#[derive( PartialEq, Debug)]
pub enum RustValue {
    F(f64),
    B(bool),
    S(String),
    U,
    N,
    O(HashMap<String,RustValue>)
}

pub trait Thing {
    fn get(&self, key:&mut String ) -> Thing;
    fn keys(&self) -> RustValue;
    fn set(&self,key: &mut String, value:&mut Thing);
    fn value(&self) -> RustValue;
}

pub trait Evaluable {
    fn eval(&self,context: &mut Thing) -> Thing;
}

pub struct Undef;

impl Thing for Undef{
    fn get(&self, key:&mut String ) -> Thing{
        return Undef::new();
    }
    fn keys(&self) -> RustValue{
        let mut keyList:HashMap<String,RustValue>=HashMap::new();
        let mut i=0;
        keyList.insert("length".to_string(),RustValue::F(i as f64);
        return RustValue::O(keyList);
    }
    fn set(&self,key: &mut String, value:&mut Thing){
        return
    }
    fn value(&self) -> RustValue{
        return RustValue::U;
    }
}
impl Undef{
    fn new() -> Undef{
        Undef {}
    }
}

pub struct Object {
    map:HashMap<String,Object>
}

impl Thing for Object{
    fn get(&self, key:&mut String) -> Thing{
        if !self.map.contains_key(key){
            return Undef::new();
        }
        return self.map.get(key);
    }
    fn keys(&self) -> RustValue{
        let mut keyList:HashMap<String,RustValue>=HashMap::new();
        let mut i=0;
        for key in self.map.keys(){
            keyList.insert(i.to_string(),RustValue::S(key.to_string()));
            i=i+1;
        }
        keyList.insert("length",RustValue::F(i));
        return RustValue::O(keyList);
    }
    fn set(&self,key: &mut String, value:&mut Thing){
        self.map.insert(key,value);
    }
    fn value(&self) -> RustValue{
        let mut converted:HashMap<String,RustValue>=HashMap::new();
        for (k,v) in self.map{
            converted.insert(k,v.value());
        }
        return RustValue::O(converted);
    }
}

impl Object{
    fn new(map:HashMap<String,Object>) -> Object{
        Object {map:map}
    }
    fn empty() -> Object{
        Object {map:HashMap::new()}
    }
}

pub struct Number {
    v:f64
}

impl Thing for Number{
    fn get(&self, key:&mut String) -> Thing{
        return Undef;
    }
    fn keys(&self) -> RustValue{
        let mut keyList:HashMap<String,RustValue>=HashMap::new();
        let mut i=0;
        keyList.insert("length",RustValue::F(i));
        return RustValue::O(keyList);
    }
    fn set(&self,key: &mut String, value:&mut Thing){
        return
    }
    fn value(&self) -> RustValue{
        return RustValue::F(self.v);
    }
}

impl Number{
    fn new(n:f64) -> Number{
        Number {v:n}
    }
}

/*
struct Function {
    protype
}

impl Thing for Function{
    fn get(&self, key:&mut str ) -> Thing;
    fn keys(&self) -> RustValue::O;
    fn set(&self,key: &mut str, value:&mut Thing);
    fn value(&self) -> RustValue;
}
*/
