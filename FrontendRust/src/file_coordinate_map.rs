// From Frontend/Utils/src/dev/vale/CodeHierarchy.scala lines 104-189
// FileCoordinateMap implements IPackageResolver and stores file contents by coordinate

use crate::parsing::ast::{FileCoordinate, PackageCoordinate};
use crate::utils::IPackageResolver;
use std::collections::HashMap;

// From CodeHierarchy.scala lines 104-189
pub struct FileCoordinateMap<Contents> {
    package_coord_to_file_coords: HashMap<PackageCoordinate, Vec<FileCoordinate>>,
    file_coord_to_contents: HashMap<FileCoordinate, Contents>,
}

impl<Contents: Clone> FileCoordinateMap<Contents> {
    pub fn new() -> Self {
        FileCoordinateMap {
            package_coord_to_file_coords: HashMap::new(),
            file_coord_to_contents: HashMap::new(),
        }
    }
    
    // From CodeHierarchy.scala lines 112-114
    pub fn apply(&self, coord: &FileCoordinate) -> &Contents {
        self.file_coord_to_contents.get(coord)
            .expect("FileCoordinateMap::apply - coordinate not found")
    }
    
    // From CodeHierarchy.scala lines 118-127: putPackage
    pub fn put_package(
        &mut self,
        package_coord: PackageCoordinate,
        new_file_coord_to_contents: HashMap<FileCoordinate, Contents>,
    ) {
        let file_coords: Vec<FileCoordinate> = new_file_coord_to_contents.keys().cloned().collect();
        self.package_coord_to_file_coords.insert(package_coord, file_coords);
        
        for (file_coord, contents) in new_file_coord_to_contents {
            self.file_coord_to_contents.insert(file_coord, contents);
        }
    }
    
    // From CodeHierarchy.scala lines 129-135: put
    pub fn put(&mut self, file_coord: FileCoordinate, contents: Contents) {
        assert!(!self.file_coord_to_contents.contains_key(&file_coord),
            "FileCoordinateMap::put - file coordinate already exists");
        
        self.file_coord_to_contents.insert(file_coord.clone(), contents);
        
        let package_coord = file_coord.package_coord.clone();
        let file_coords = self.package_coord_to_file_coords
            .entry(package_coord)
            .or_insert_with(Vec::new);
        file_coords.push(file_coord);
    }
    
    // From CodeHierarchy.scala lines 137-143: map
    pub fn map<T, F>(&self, func: F) -> FileCoordinateMap<T>
    where
        F: Fn(&FileCoordinate, &Contents) -> T,
        T: Clone,
    {
        let mut result_file_coord_to_contents = HashMap::new();
        for (file_coord, contents) in &self.file_coord_to_contents {
            result_file_coord_to_contents.insert(file_coord.clone(), func(file_coord, contents));
        }
        
        FileCoordinateMap {
            package_coord_to_file_coords: self.package_coord_to_file_coords.clone(),
            file_coord_to_contents: result_file_coord_to_contents,
        }
    }
    
    // From CodeHierarchy.scala lines 145-149: flatMap
    pub fn flat_map<T, F>(&self, func: F) -> Vec<T>
    where
        F: Fn(&FileCoordinate, &Contents) -> T,
    {
        self.file_coord_to_contents
            .iter()
            .map(|(file_coord, contents)| func(file_coord, contents))
            .collect()
    }
    
    // From CodeHierarchy.scala lines 151-153: expectOne
    pub fn expect_one(&self) -> &Contents {
        assert!(self.file_coord_to_contents.len() == 1,
            "FileCoordinateMap::expect_one - expected exactly one entry");
        self.file_coord_to_contents.values().next().unwrap()
    }
}

// From CodeHierarchy.scala lines 109, 178-188: IPackageResolver implementation
impl<Contents: Clone> IPackageResolver<HashMap<String, Contents>> for FileCoordinateMap<Contents> {
    fn resolve(&self, package_coord: &PackageCoordinate) -> Option<HashMap<String, Contents>> {
        self.package_coord_to_file_coords
            .get(package_coord)
            .map(|file_coords| {
                file_coords
                    .iter()
                    .map(|file_coord| {
                        let contents = self.file_coord_to_contents.get(file_coord)
                            .expect("FileCoordinateMap::resolve - file coord not found in contents");
                        (file_coord.filepath.clone(), contents.clone())
                    })
                    .collect()
            })
    }
}

