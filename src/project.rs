use {CameraCalibration, Error, MountCalibration, Result, ScanPosition, utils};
use element::Extension;
use nalgebra::Projective3;
use scan_position::Image;
use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use xmltree::Element;

/// A RiSCAN Pro project.
///
/// These are always created by pointing at a path, either the `.RiSCAN` path or the `project.rsp`
/// in that directory:
///
/// ```
/// use riscan_pro::Project;
/// let project1 = Project::from_path("data/project.RiSCAN").unwrap();
/// let project2 = Project::from_path("data/project.RiSCAN/project.rsp").unwrap();
/// assert_eq!(project1, project2);
/// ```
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Project {
    /// The camera calibrations, by name.
    pub camera_calibrations: HashMap<String, CameraCalibration>,
    /// The camera mount calibrations, by name.
    pub mount_calibrations: HashMap<String, MountCalibration>,
    /// The scan positions, by name.
    pub scan_positions: HashMap<String, ScanPosition>,
    /// The project's own position.
    pub pop: Projective3<f64>,
}

impl Project {
    /// Creates a project from a filesystem path.
    ///
    /// This path can be either the `.RiSCAN` directory or the contained `project.rsp`.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::Project;
    /// let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Project> {
        use std::fs::File;

        let path = rsp_path(path)?;
        let file = File::open(path)?;
        Project::from_read(file)
    }

    /// Returns a scan position, as determined by the path.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::Project;
    /// let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// let scan_position1 = project.scan_positions.get("SP01").unwrap();
    /// let path = "data/project.RiSCAN/SCANS/SP01/SCANPOSIMAGES/SP01 - Image001.csv";
    /// let scan_position2 = project.scan_position_from_path(path).unwrap();
    /// assert_eq!(scan_position1, scan_position2);
    /// ```
    pub fn scan_position_from_path<P: AsRef<Path>>(&self, path: P) -> Result<&ScanPosition> {
        path.as_ref()
            .file_stem()
            .map(|file_stem| file_stem.to_string_lossy())
            .and_then(|file_stem| {
                file_stem.split(" - ").next().and_then(|name| {
                    self.scan_positions.get(name)
                })
            })
            .ok_or(Error::ScanPositionFromPath(path.as_ref().to_path_buf()))
    }

    fn from_read<R: Read>(read: R) -> Result<Project> {
        let xml = Element::parse(read)?;

        let camera_calibrations = xml.children("calibrations/camcalibs/camcalib_opencv")?
            .iter()
            .map(|camcalib_opencv| {
                let camera_calibration = CameraCalibration::from_element(camcalib_opencv)?;
                Ok((camera_calibration.name.clone(), camera_calibration))
            })
            .collect::<Result<HashMap<_, _>>>()?;
        let mount_calibrations = xml.children("calibrations/mountcalibs/mountcalib")?
            .iter()
            .map(|mountcalib| {
                let mount_calibration = MountCalibration::from_element(mountcalib)?;
                Ok((mount_calibration.name.clone(), mount_calibration))
            })
            .collect::<Result<HashMap<_, _>>>()?;
        let scan_positions = xml.children("scanpositions/scanposition")?
            .iter()
            .map(|scanposition| {
                let scan_position = ScanPosition::from_element(scanposition)?;
                Ok((scan_position.name.clone(), scan_position))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        Ok(Project {
            camera_calibrations: camera_calibrations,
            mount_calibrations: mount_calibrations,
            scan_positions: scan_positions,
            pop: utils::parse_projective3(xml.child("pop/matrix")?.as_str()?)?,
        })
    }
}

impl CameraCalibration {
    fn from_element(element: &Element) -> Result<CameraCalibration> {
        let version = element.child("version")?.as_str()?;
        if version == "2" {
            Ok(CameraCalibration {
                name: element.child("name")?.as_str()?.to_string(),
                cx: element.child("internal_opencv/cx")?.parse_text()?,
                cy: element.child("internal_opencv/cy")?.parse_text()?,
                fx: element.child("internal_opencv/fx")?.parse_text()?,
                fy: element.child("internal_opencv/fy")?.parse_text()?,
                k1: element.child("internal_opencv/k1")?.parse_text()?,
                k2: element.child("internal_opencv/k2")?.parse_text()?,
                k3: element.child("internal_opencv/k3")?.parse_text()?,
                k4: element.child("internal_opencv/k4")?.parse_text()?,
                p1: element.child("internal_opencv/p1")?.parse_text()?,
                p2: element.child("internal_opencv/p2")?.parse_text()?,
                tan_max_horz: element.child("angle_extents/tan_max_horz")?.parse_text()?,
                tan_max_vert: element.child("angle_extents/tan_max_vert")?.parse_text()?,
                tan_min_horz: element.child("angle_extents/tan_min_horz")?.parse_text()?,
                tan_min_vert: element.child("angle_extents/tan_min_vert")?.parse_text()?,
                width: element.child("intrinsic_opencv/nx")?.parse_text()?,
                height: element.child("intrinsic_opencv/ny")?.parse_text()?,
            })
        } else {
            Err(Error::CameraCalibrationVersion(version.to_string()))
        }
    }
}

impl MountCalibration {
    fn from_element(element: &Element) -> Result<MountCalibration> {
        Ok(MountCalibration {
            name: element.child("name")?.as_str()?.to_string(),
            matrix: utils::parse_projective3(element.child("matrix")?.as_str()?)?,
        })
    }
}

impl ScanPosition {
    fn from_element(element: &Element) -> Result<ScanPosition> {
        Ok(ScanPosition {
            name: element.child("name")?.as_str()?.to_string(),
            images: element
                .children("scanposimages/scanposimage")?
                .iter()
                .map(|scanposimage| {
                    let image = Image::from_element(scanposimage)?;
                    Ok((image.name.clone(), image))
                })
                .collect::<Result<HashMap<_, _>>>()?,
            sop: utils::parse_projective3(element.child("sop/matrix")?.as_str()?)?,
        })
    }
}

impl Image {
    fn from_element(element: &Element) -> Result<Image> {
        Ok(Image {
            name: element.child("name")?.as_str()?.to_string(),
            cop: utils::parse_projective3(element.child("cop/matrix")?.as_str()?)?,
            camera_calibration_name: element.child("camcalib_ref")?.noderef()?.to_string(),
            mount_calibration_name: element.child("mountcalib_ref")?.noderef()?.to_string(),
        })
    }
}

fn rsp_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    if let Some(extension) = path.as_ref().extension() {
        let mut path_buf = path.as_ref().to_path_buf();
        if extension == "rsp" {
            return Ok(path_buf);
        } else if extension == "RiSCAN" {
            path_buf.push("project.rsp");
            return Ok(path_buf);
        }
    }
    let mut path_buf = PathBuf::new();
    for component in path.as_ref().iter() {
        path_buf.push(component);
        if Path::new(component)
            .extension()
            .map(|e| e == "RiSCAN")
            .unwrap_or(false)
        {
            return rsp_path(path_buf);
        }
    }
    Err(Error::ProjectPath(path_buf))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_path() {
        Project::from_path("data/project.RiSCAN").unwrap();
        Project::from_path("data/project.RiSCAN/project.rsp").unwrap();
        Project::from_path("data/project.RiSCAN/SCANS").unwrap();
        assert!(Project::from_path("data").is_err());
    }

    #[test]
    fn mount_calibrations() {
        use utils;

        let project = Project::from_path("data/project.RiSCAN").unwrap();
        let mount_calibration = project
            .mount_calibrations
            .get("Infratec_VarioCAM_HD_15mm_11-16-2015_Preston")
            .unwrap();
        let matrix = utils::parse_projective3("-0.010877741999999997 -0.003724941 -0.999933898 0.18508641   0.019274697 0.999806486 -0.0039341460000000013 0.000460517   0.99975505 -0.019316217 -0.01080384 -0.092802787   0 0 0 1").unwrap();
        assert_eq!(matrix, **mount_calibration);
    }

    #[test]
    fn scan_position_from_path() {
        let project = Project::from_path("data/project.RiSCAN").unwrap();
        let scan_position1 = project.scan_positions.get("SP01").unwrap();
        let scan_position2 = project
            .scan_position_from_path(
                "data/project.RiSCAN/SCANS/SP01/SCANPOSIMAGES/SP01 - Image001.csv",
            )
            .unwrap();
        assert_eq!(scan_position1, scan_position2);
    }
}
