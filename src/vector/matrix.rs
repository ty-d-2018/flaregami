use std::collections::HashMap;

#[derive(Clone)]
pub enum ProtoType{
    One,
    Two,
    Three,
    Four,
    N,
}

pub type Matrix = ProtoMatrix;
pub type Dimension = ProtoType;
pub type Numeric = f32;

#[derive(Clone)]
pub struct ProtoMatrix{
    mark: ProtoType,
    size: (usize, usize),
    numbers: HashMap<usize, Vec<Numeric>>,
}

#[derive(Clone)]
struct Template{
    pub d_x: usize,
    pub d_y: usize,
    pub numbers: Vec<Numeric>,
}

impl ProtoMatrix{
    fn new(template: Template, prototype: &ProtoType) -> Matrix{
        let mut mmap: HashMap<usize, Vec<Numeric>> = HashMap::new();
        let mut dimension: Vec<Numeric> = Vec::new();
        let mut column: usize = 0;
        let mut row: usize = 0;
        for arrow in template.numbers{
            if row >= template.d_y{
                break;
            }

            dimension.push(arrow);

            if !(column < template.d_x){
                mmap.insert(row, dimension);
                dimension = Vec::new();
                row += 1;
                column = 0;
            }else{
                column += 1;
            }
        }

        Matrix{
            mark: prototype.clone(),
            size: (template.d_x, template.d_y),
            numbers: mmap,
        }
    }
    pub fn set_line(d_x: usize, d_y: usize, number_list: &Vec<Numeric>, prototype: &ProtoType) -> Matrix{
        let template: Template = Template{
            d_x,
            d_y,
            numbers: number_list.clone(),
        };
        let matrix: Matrix = Matrix::new(template, prototype);

        matrix
    }
}