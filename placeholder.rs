pub fn get_item(&self, element: &str) -> Result<String, &str> {
    // Getting the x coordinate:
    let mut x_cor: usize = 0;
    for i in self.header.iter() {
        if i == &element[0..1] {
            break;
        }
        x_cor += 1;
    }

    // Getting the y coordinate:
    let y_cor = match &element[1..element.len()].parse::<usize>() {
        Ok(val) => *val - 1,
        Err(err) => return Err("Unparsable 'y' coordinate!"),
    };

    match self.body.get(y_cor) {
        Some(row) => {
            match row.get(x_cor) {
                Some(s) => return Ok(s.to_string()),
                None => return Err("[ERROR] Out of bounds 'x' coordinate..."), 
            }
        },
        None => return Err("[ERROR] Out of bounds 'y' coordinate..."),
    }
}