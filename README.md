# xml-de-serializer-test

Experimenting wih rust/serde de/serialization in combination with quick-xml

```xml
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

```
