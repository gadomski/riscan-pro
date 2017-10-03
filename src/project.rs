use {Error, Matrix, Result, Scan, ScanPosition, Tiepoint};
use nalgebra::Matrix4;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use xml::reader::{EventReader, XmlEvent};

/// A RiSCAN Pro project.
#[derive(Clone, Debug)]
pub struct Project {
    path: Option<PathBuf>,
    pop: Matrix4<f64>,
    scan_positions: HashMap<String, ScanPosition>,
    tpl_glcs: Vec<Tiepoint>,
}

impl Project {
    /// Reads a project from a path.
    ///
    /// This path can either be the `.RiSCAN` directory, or the underlying `project.rsp` file.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::Project;
    /// let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// let project = Project::from_path("data/project.RiSCAN/project.rsp").unwrap();
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Project> {
        let mut path = path.as_ref().to_path_buf();
        if fs::metadata(&path)?.is_dir() {
            path.push("project.rsp");
        }
        let ref mut reader = EventReader::new(BufReader::new(File::open(&path)?));
        let mut project = Project::new();
        path.pop();
        project.path = Some(path);

        if let Some(name) = next_element_name(reader)? {
            if name != "project" {
                return Err(Error::XmlRead(format!("invalid root element: {}", name)));
            }
        } else {
            return Err(Error::XmlRead("empty xml file".to_string()));
        }

        while let Some(name) = next_element_name(reader)? {
            match name.as_str() {
                "pop" => {
                    project.set_pop(read_matrix(reader)?);
                }
                "scanpositions" => {
                    while let Some(scan_position) = read_scan_position(reader)? {
                        project.add_scan_position(scan_position);
                    }
                }
                "tpl_glcs" => {
                    while let Some(tiepoint) = read_tp_glcs(reader)? {
                        project.add_tp_glcs(tiepoint);
                    }
                }
                _ => consume_branch(reader)?,
            }
        }

        Ok(project)
    }

    /// Creates a new, default project.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::Project;
    /// let project = Project::new();
    /// ```
    pub fn new() -> Project {
        Project {
            scan_positions: HashMap::new(),
            path: None,
            pop: Matrix4::identity(),
            tpl_glcs: Vec::new(),
        }
    }

    /// Returns this project's POP.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::Project;
    /// # let mut project = Project::new();
    /// let pop = project.pop();
    /// ```
    pub fn pop(&self) -> Matrix {
        self.pop
    }

    /// Sets this project's POP matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::{Project, Matrix};
    /// # let mut project = Project::new();
    /// project.set_pop(Matrix::new(1., 0., 0., 1.,
    ///                             0., 1., 0., 2.,
    ///                             0., 0., 1., 3.,
    ///                             0., 0., 0., 1.));
    /// ```
    pub fn set_pop(&mut self, pop: Matrix) {
        for scan_position in self.scan_positions.values_mut() {
            scan_position.set_pop(pop);
        }
        self.pop = pop;
    }

    /// Returns a scan position with the given name or scan.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::Project;
    /// let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// assert!(project.scan_position("SP01").is_some());
    /// assert!(project.scan_position("151120_150404").is_some());
    /// assert!(project.scan_position("SCANS/SP01/SINGLESCANS/151120_150404.rxp").is_some());
    /// assert!(project.scan_position("151120_150404.mta.rxp").is_some());
    /// ```
    pub fn scan_position(&self, name: &str) -> Option<&ScanPosition> {
        self.scan_positions.get(name).or_else(|| {
            if let Some(file_name) = Path::new(name).file_name() {
                if let Some(name) = file_name.to_string_lossy().split('.').next() {
                    return self.scan_positions.values().find(|scan_position| {
                                                                 scan_position.scan(name).is_some()
                                                             });
                }
            }
            None
        })
    }

    /// Returns a vector of all the scan positions in this scan.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::Project;
    /// let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// let scan_positions = project.scan_positions();
    /// ```
    pub fn scan_positions(&self) -> Vec<&ScanPosition> {
        self.scan_positions.values().collect()
    }

    /// Adds a scan position to this project.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::{Project, ScanPosition};
    /// let mut project = Project::new();
    /// let scan_position = ScanPosition::new();
    /// project.add_scan_position(scan_position);
    /// ```
    pub fn add_scan_position(&mut self, mut scan_position: ScanPosition) {
        scan_position.set_pop(self.pop);
        self.scan_positions.insert(scan_position.name().to_string(), scan_position);
    }

    /// Returns this project's (optional) path as a reference.
    ///
    /// The project path is the `.RiSCAN` directory, not the XML file.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::Project;
    /// let project = Project::from_path("data/project.RiSCAN/project.rsp").unwrap();
    /// assert_eq!("data/project.RiSCAN", project.path().unwrap().to_string_lossy());
    /// ```
    pub fn path(&self) -> Option<&Path> {
        self.path.as_ref().map(|path_buf| path_buf.as_path())
    }

    /// Adds a GLCS tiepoint.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::{Project, Tiepoint};
    /// let tiepoint: Tiepoint = Default::default();
    /// let mut project = Project::new();
    /// project.add_tp_glcs(tiepoint);
    /// ```
    pub fn add_tp_glcs(&mut self, tiepoint: Tiepoint) {
        self.tpl_glcs.push(tiepoint);
    }

    /// Returns this project's global tiepoints.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::Project;
    /// let project = Project::from_path("data/project.RiSCAN/project.rsp").unwrap();
    /// let tiepoints = project.tpl_glcs();
    /// ```
    pub fn tpl_glcs(&self) -> &Vec<Tiepoint> {
        &self.tpl_glcs
    }
}

fn next_element_name<R: Read>(reader: &mut EventReader<R>) -> Result<Option<String>> {
    loop {
        match reader.next()? {
            XmlEvent::StartElement { name, .. } => return Ok(Some(name.local_name)),
            XmlEvent::EndDocument => return Ok(None),
            _ => {}
        }
    }
}

fn consume_branch<R: Read>(reader: &mut EventReader<R>) -> Result<()> {
    let mut depth = 0;
    loop {
        match reader.next()? {
            XmlEvent::StartElement { .. } => depth += 1,
            XmlEvent::EndElement { .. } => {
                if depth == 0 {
                    return Ok(());
                } else {
                    depth -= 1;
                }
            }
            _ => {}
        }
    }
}

fn read_matrix<R: Read>(reader: &mut EventReader<R>) -> Result<Matrix> {
    loop {
        match reader.next()? {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_str() {
                    "matrix" => {
                        let matrix = read_characters(reader).and_then(|s| matrix_from_str(&s))?;
                        consume_branch(reader)?;
                        return Ok(matrix);
                    }
                    _ => consume_branch(reader)?,
                }
            }
            XmlEvent::EndElement { .. } => {
                return Err(Error::XmlRead("missing matrix element".to_string()));
            }
            _ => {}
        }
    }
}

fn read_scan_position<R: Read>(reader: &mut EventReader<R>) -> Result<Option<ScanPosition>> {
    loop {
        match reader.next()? {
            XmlEvent::StartElement { name, .. } => {
                if name.local_name != "scanposition" {
                    return Err(Error::XmlRead(format!("unexpected element name: {}", name)));
                }
                break;
            }
            XmlEvent::EndElement { .. } => return Ok(None),
            _ => {}
        }
    }
    let mut scan_position = ScanPosition::new();
    loop {
        match reader.next()? {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_str() {
                    "name" => {
                        scan_position.set_name(&read_characters(reader)?);
                    }
                    "singlescans" => {
                        while let Some(scan) = read_scan(reader)? {
                            scan_position.add_scan(scan);
                        }
                    }
                    "sop" => scan_position.set_sop(read_matrix(reader)?),
                    _ => consume_branch(reader)?,
                }
            }
            XmlEvent::EndElement { .. } => break,
            _ => {}
        }
    }
    Ok(Some(scan_position))
}

fn read_tp_glcs<R: Read>(reader: &mut EventReader<R>) -> Result<Option<Tiepoint>> {
    loop {
        match reader.next()? {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_str() {
                    "tp_glcs" => break,
                    "active" => consume_branch(reader)?,
                    _ => return Err(Error::XmlRead(format!("unexpected element name: {}", name))),
                }
            }
            XmlEvent::EndElement { .. } => return Ok(None),
            _ => {}
        }
    }
    let mut tiepoint: Tiepoint = Default::default();
    loop {
        match reader.next()? {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_str() {
                    "name" => tiepoint.name = read_characters(reader)?,
                    "height" => tiepoint.height = read_characters(reader)?.parse()?,
                    "vector" => {
                        let string = read_characters(reader)?;
                        let words = string.split_whitespace().collect::<Vec<_>>();
                        tiepoint.x = words[0].parse()?;
                        tiepoint.y = words[1].parse()?;
                        tiepoint.z = words[2].parse()?;
                    }
                    _ => consume_branch(reader)?,
                }
            }
            XmlEvent::EndElement { .. } => break,
            _ => {}
        }
    }
    Ok(Some(tiepoint))
}

fn read_scan<R: Read>(reader: &mut EventReader<R>) -> Result<Option<Scan>> {
    loop {
        match reader.next()? {
            XmlEvent::StartElement { name, .. } => {
                if name.local_name != "scan" {
                    return Err(Error::XmlRead(format!("unexpected element name: {}", name)));
                }
                break;
            }
            XmlEvent::EndElement { .. } => return Ok(None),
            _ => {}
        }
    }
    let mut scan = Scan::new();
    loop {
        match reader.next()? {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_str() {
                    "name" => {
                        scan.set_name(&read_characters(reader)?);
                    }
                    _ => consume_branch(reader)?,
                }
            }
            XmlEvent::EndElement { .. } => break,
            _ => {}
        }
    }
    Ok(Some(scan))
}

fn read_characters<R: Read>(reader: &mut EventReader<R>) -> Result<String> {
    let s = match reader.next()? {
        XmlEvent::Characters(s) => s,
        event @ _ => return Err(Error::XmlRead(format!("expected characters, got {:?}", event))),
    };
    consume_branch(reader)?;
    Ok(s)
}

fn matrix_from_str(s: &str) -> Result<Matrix> {
    let entries = s.split_whitespace()
        .map(|s| s.parse().map_err(Error::from))
        .collect::<Result<Vec<_>>>()?;
    if entries.len() != 16 {
        return Err(Error::Matrix(entries));
    }
    Ok(Matrix::new(entries[0],
                   entries[1],
                   entries[2],
                   entries[3],
                   entries[4],
                   entries[5],
                   entries[6],
                   entries[7],
                   entries[8],
                   entries[9],
                   entries[10],
                   entries[11],
                   entries[12],
                   entries[13],
                   entries[14],
                   entries[15]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ScanPosition;

    #[test]
    fn project_read_from() {
        let project = Project::from_path("data/project.RiSCAN").unwrap();
        assert_eq!(0.99566497679815923, project.pop()[(0, 0)]);
        assert_eq!(2, project.scan_positions().len());
        assert!(project.scan_position("SP01").is_some());
        assert!(project.scan_position("SP02").is_some());
        assert!(project.scan_position("SP03").is_none());
    }

    #[test]
    fn project_add_scan_position() {
        let mut pop = Matrix4::identity();
        pop[(0, 3)] = 1.;
        let mut project = Project::new();
        project.set_pop(pop);
        let mut scan_position = ScanPosition::new();
        scan_position.set_name("ScanPos001");
        project.add_scan_position(scan_position);
        assert_eq!(pop, project.scan_position("ScanPos001").unwrap().pop());
    }

    #[test]
    fn project_set_pop() {
        let mut project = Project::new();
        let mut scan_position = ScanPosition::new();
        scan_position.set_name("ScanPos001");
        project.add_scan_position(scan_position);
        let mut pop = Matrix4::identity();
        pop[(0, 3)] = 1.;
        project.set_pop(pop);
        assert_eq!(pop, project.scan_position("ScanPos001").unwrap().pop());

    }
}
