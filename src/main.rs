use quick_xml::de::from_str;
use quick_xml::se::to_string;
use quick_xml::serde_helpers::text_content::deserialize;
use serde::de::DeserializeOwned;
use serde::de::{self, MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt; // Import this

/**

<?xml version="1.0"?>
<Outer>
  <Inner>
    <A>
      <Value>thats a string</Value>
    </A>
  </Inner>
</Outer>

--

<?xml version="1.0"?>
<Outer>
  <Inner>
    <B>
      <Value>47</Value>
    </B>
  </Inner>
</Outer>

*/
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
enum AnyNode {
    A(A),
    B(B),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct Outer<T> {
    inner: Inner<T>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct Inner<T> {
    data: T,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct A {
    some_value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct B {
    some_other_value: u32,
}

fn main() {
    println!("This project currently only has unit tests!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_de_serialize_a_1() {
        let outer_a = Outer {
            inner: Inner {
                data: A {
                    some_value: String::from("thats a string"),
                },
            },
        };

        let xml_outer_a = to_string(&outer_a).unwrap();
        println!("xml_outer_a: {:#?}", xml_outer_a);

        let outer_a_2 = from_str::<Outer<A>>(&xml_outer_a).unwrap();

        assert_eq!(
            outer_a.inner.data.some_value,
            outer_a_2.inner.data.some_value
        );
    }

    #[test]
    pub fn test_de_serialize_b_1() {
        let outer_b = Outer {
            inner: Inner {
                data: B {
                    some_other_value: 47,
                },
            },
        };

        let xml_outer_b = to_string(&outer_b).unwrap();
        println!("xml_outer_b: {:#?}", xml_outer_b);

        let outer_b_2 = from_str::<Outer<B>>(&xml_outer_b).unwrap();

        assert_eq!(
            outer_b.inner.data.some_other_value,
            outer_b_2.inner.data.some_other_value
        );
    }

    #[test]
    pub fn test_de_serialize_message_1() {
        let a = A {
            some_value: String::from("thats a string"),
        };

        let outer_message_a = Outer {
            inner: Inner { data: a },
        };

        let xml_outer_message_a = to_string(&outer_message_a).unwrap();
        println!("xml_outer_message_a: {:#?}", xml_outer_message_a);

        let data = from_str::<Outer<AnyNode>>(&xml_outer_message_a).unwrap();

        // let outer_a_2 = from_str::<Outer<A>>(&xml_outer_a).unwrap();
        //
        // assert_eq!(
        //     outer_a.inner.data.some_value,
        //     outer_a_2.inner.data.some_value
        // );
    }
}
