//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use support::Error;
use std::io::{BufRead, BufReader};
use byteorder::ReadBytesExt;
use support::resource::Chunk;
use std::fs::File;
use support;

// MAT file is an index of: material internal name, PIX file name and TAB file name.
#[derive(Default)]
pub struct Material {
    params: [f32; 12],
    name: String,
    pixelmap_name: String,
    rendertab_name: String,

}

impl Material {
    pub fn load<R: ReadBytesExt + BufRead>(rdr: &mut R) -> Result<Material, Error> {
        let mut mat = Material::default();

        // Read chunks until last chunk is encountered.
        // Certain chunks initialize certain properties.
        loop {
            let c = Chunk::load(rdr)?;
            match c {
                Chunk::MaterialDesc { name, params } => {
                    mat.params = params;
                    mat.name = name;
                },
                Chunk::PixelmapRef(name) => {
                    mat.pixelmap_name = name;
                },
                Chunk::RenderTabRef(name) => {
                    mat.rendertab_name = name;
                },
                Chunk::Null() => break,
                Chunk::FileHeader { file_type } => {
                    if file_type != support::MATERIAL_FILE_TYPE {
                        panic!("Invalid material file type {}", file_type);
                    }
                },
                _ => unimplemented!(), // unexpected type here
            }
        }

        Ok(mat)
    }

    pub fn load_from(fname: String) -> Result<Material, Error> {
        let file = File::open(fname)?;
        let mut file = BufReader::new(file);
        Material::load(&mut file)
    }
}
