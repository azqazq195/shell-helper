use std::{fs, io};
use std::path::{Path, PathBuf};
use crate::command::copy_command::copy::CopyArgs;

pub fn copy(args: &CopyArgs) -> io::Result<()> {
    let from_path = PathBuf::from(&args.from);
    let to_path = PathBuf::from(&args.to);

    match from_path.is_dir() {
        true => copy_dir(&from_path, &to_path),
        false => copy_file(&from_path, &to_path),
    }
}

fn copy_file(from: &Path, to: &Path) -> io::Result<()> {
    if let Some(parent) = to.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    fs::copy(from, to)?;
    Ok(())
}

fn copy_dir(from: &Path, to: &Path) -> io::Result<()> {
    if !to.exists() {
        fs::create_dir_all(to)?;
    }

    let entries = fs::read_dir(from)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        let dest_path = to.join(file_name);

        match path.is_dir() {
            true => copy_dir(&path, &dest_path)?,
            false => copy_file(&path, &dest_path)?,
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::{tempdir, tempdir_in};

    const TEST_FILE_CONTENT: &str = "This is a test file.";

    #[test]
    fn test_copy_file() -> io::Result<()> {
        let temp_dir = tempdir()?;
        let source_path = temp_dir.path().join("source.txt");
        let mut source_file = File::create(&source_path)?;
        write!(source_file, "{}", TEST_FILE_CONTENT)?;

        let dest_path = temp_dir.path().join("dest.txt");
        copy_file(&source_path, &dest_path)?;

        let copied_content = fs::read_to_string(&dest_path)?;
        assert_eq!(copied_content, TEST_FILE_CONTENT);

        Ok(())
    }

    #[test]
    fn test_copy_dir() -> io::Result<()> {
        // 임시 폴더 구조 생성
        let temp_dir_depth_1 = tempdir()?;
        let temp_dir_depth_2 = tempdir_in(temp_dir_depth_1.path())?;
        let temp_dir_depth_3 = tempdir_in(temp_dir_depth_2.path())?;
        let temp_dir_depth_4 = tempdir_in(temp_dir_depth_3.path())?;
        let source_path = temp_dir_depth_4.path().join("source.txt");
        let mut source_file = File::create(&source_path)?;
        write!(source_file, "{}", TEST_FILE_CONTENT)?;

        // 복사 대상 폴더 설정
        let target_dir = tempdir()?;
        let dest_path = target_dir.path().join("copied_dir");

        // 폴더 복사 실행
        copy_dir(temp_dir_depth_1.path(), &dest_path)?;

        // 복사 검증
        let dest_file_path = dest_path
            .join(temp_dir_depth_2.path().file_name().unwrap())
            .join(temp_dir_depth_3.path().file_name().unwrap())
            .join(temp_dir_depth_4.path().file_name().unwrap())
            .join("source.txt");

        assert!(dest_file_path.exists());

        let copied_content = fs::read_to_string(dest_file_path)?;
        assert_eq!(copied_content, TEST_FILE_CONTENT);

        Ok(())
    }
}
