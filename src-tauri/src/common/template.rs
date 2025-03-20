use std::collections::HashMap;

pub fn template_replace(template: &str, params: &HashMap<String, String>) -> String {
  let mut result = template.to_string();
  for (key, value) in params {
      let key = format!("${{{}}}", key); // key=output时，结果为：${output}
      result = result.replace(&key, value);
  }
  result
}

pub fn template_replace_single(template: &str, key: &str, value: &str) -> String {
  let mut params = HashMap::new();
  params.insert(key.to_string(), value.to_string());
  template_replace(template, &params)
}