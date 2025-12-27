use super::ast::*;

/// Von printer for JSON serialization
/// Matches Scala's VonPrinter with JsonSyntax
pub struct VonPrinter {
    #[allow(dead_code)]
    line_width: usize,
}

impl VonPrinter {
    pub fn new() -> Self {
        VonPrinter { line_width: 120 }
    }

    pub fn print(&self, data: &IVonData) -> String {
        let mut result = String::new();
        self.print_multiline(&mut result, data, 0);
        result
    }

    fn escape(&self, value: &str) -> String {
        value
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t")
    }

    fn print_multiline(&self, builder: &mut String, data: &IVonData, indentation: usize) {
        match data {
            IVonData::Int(VonInt { value }) => {
                builder.push_str(&value.to_string());
            }
            IVonData::Float(VonFloat { value }) => {
                builder.push_str(&value.to_string());
            }
            IVonData::Bool(VonBool { value }) => {
                builder.push_str(&value.to_string());
            }
            IVonData::Str(VonStr { value }) => {
                builder.push('"');
                builder.push_str(&self.escape(value));
                builder.push('"');
            }
            IVonData::Object(obj) => {
                self.print_object_multiline(builder, obj, indentation);
            }
            IVonData::Array(arr) => {
                self.print_array_multiline(builder, arr, indentation);
            }
        }
    }

    fn print_object_multiline(&self, builder: &mut String, obj: &VonObject, indentation: usize) {
        let members = &obj.members;

        // JSON: {"__type": "TypeName", "field1": value1, ...}
        builder.push_str("{\"__type\": \"");
        builder.push_str(&self.escape(&obj.tyype));
        builder.push('"');
        
        if !members.is_empty() {
            builder.push_str(", ");
            builder.push('\n');

            for (index, member) in members.iter().enumerate() {
                self.print_indent(builder, indentation + 1);
                self.print_member_multiline(builder, member, indentation + 1);
                if index < members.len() - 1 {
                    builder.push(',');
                }
                builder.push('\n');
            }
            
            self.print_indent(builder, indentation);
        }
        
        builder.push('}');
    }

    fn print_array_multiline(&self, builder: &mut String, arr: &VonArray, indentation: usize) {
        let members = &arr.members;

        builder.push('[');
        
        if !members.is_empty() {
            builder.push('\n');

            for (index, member) in members.iter().enumerate() {
                self.print_indent(builder, indentation + 1);
                self.print_multiline(builder, member, indentation + 1);
                if index < members.len() - 1 {
                    builder.push(',');
                }
                builder.push('\n');
            }
            
            self.print_indent(builder, indentation);
        }
        
        builder.push(']');
    }

    fn print_member_multiline(&self, builder: &mut String, member: &VonMember, indentation: usize) {
        builder.push('"');
        builder.push_str(&member.field_name);
        builder.push_str("\": ");
        self.print_multiline(builder, &member.value, indentation);
    }

    fn print_indent(&self, builder: &mut String, indentation: usize) {
        for _ in 0..indentation {
            builder.push_str("  ");
        }
    }
}

impl Default for VonPrinter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_object() {
        let printer = VonPrinter::new();
        let obj = IVonData::object(
            "TestType".to_string(),
            vec![
                VonMember::new("field1".to_string(), IVonData::int(42)),
                VonMember::new("field2".to_string(), IVonData::str("hello".to_string())),
            ],
        );
        
        let json = printer.print(&obj);
        assert!(json.contains("\"__type\": \"TestType\""));
        assert!(json.contains("\"field1\": 42"));
        assert!(json.contains("\"field2\": \"hello\""));
    }

    #[test]
    fn test_array() {
        let printer = VonPrinter::new();
        let arr = IVonData::array(vec![
            IVonData::int(1),
            IVonData::int(2),
            IVonData::int(3),
        ]);
        
        let json = printer.print(&arr);
        assert!(json.contains("["));
        assert!(json.contains("1"));
        assert!(json.contains("2"));
        assert!(json.contains("3"));
        assert!(json.contains("]"));
    }
}

