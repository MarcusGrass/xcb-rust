## Heuristics for converting the xsd to structs
All ComplexTypes are structs, their name is derived 
from their parent element, or their "name" attr

Choices are difficult
- If it has 1 element, it's transparent, look first at the choice's min/max_occurs 
if those don't exist, look at the child.
- Otherwise it's an enum

Groups are structs if declared at the top level, otherwise always references

Elements are the most difficult.
- If they contain a complex type they are a struct and always has a name
- If they have a ref, they are referencing some other struct 
- If they have a name and a type they must be a part of a choice

Sequences are transparent. Always means that we're parsing the fields of something

If an element is in a sequence, it can have a min/max-occurs

Attributes are fields
- If they have no inner tags k,v, and use is in the attributes of the tag
- If they have inner tags v are always a string

SimpleContent means that they have a value the type is the Base,

ComplexContent have a value that's a ref to some other struct

Strategy:
* 
