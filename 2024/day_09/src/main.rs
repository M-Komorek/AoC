use std::collections::HashMap;
use std::fs;
use std::io::Result;

fn read_data(file_path: &str) -> Result<Vec<u32>> {
    let data = fs::read_to_string(file_path)?;
    let data = data.trim_end();
    let disk_map: Vec<u32> = data.chars().map(|ch| ch.to_digit(10).unwrap()).collect();
    Ok(disk_map)
}

fn parse_disk_map(disk_map: Vec<u32>) -> Vec<Option<u32>> {
    let mut chunks = Vec::new();

    let mut id = 0;

    for (index, memory_len) in disk_map.iter().enumerate() {
        for _ in 0..*memory_len {
            if index % 2 == 0 {
                chunks.push(Some(id));
            } else {
                chunks.push(None);
            }
        }

        if index % 2 == 0 {
            id += 1;
        }
    }

    chunks
}

fn part_1(file_path: &str) -> Result<usize> {
    let disk_map = read_data(file_path)?;
    let mut chunks = parse_disk_map(disk_map);

    let mut left_ptr = 0;
    let mut right_ptr = chunks.len() - 1;

    while left_ptr < right_ptr {
        if chunks[left_ptr].is_none() {
            while chunks[right_ptr].is_none() {
                right_ptr -= 1;
            }

            chunks.swap(left_ptr, right_ptr);
            right_ptr -= 1;
        }

        left_ptr += 1;
    }

    let checksum: usize = chunks
        .iter()
        .enumerate()
        .filter_map(|(pos, file)| file.map(|id| pos * id as usize))
        .sum();

    Ok(checksum)
}

fn check_for_free_space(chunks: &Vec<Option<u32>>, space_needed: u32) -> Option<usize> {
    let mut current_none_count = 0;

    for i in 0..chunks.len() {
        if chunks[i].is_none() {
            current_none_count += 1;
        } else {
            current_none_count = 0;
        }

        if current_none_count == space_needed {
            return Some(i as usize - (space_needed as usize) + 1);
        }
    }

    None
}

fn get_file_id_to_size(chunks: &Vec<Option<u32>>) -> HashMap<u32, u32> {
    let mut file_id_to_size = HashMap::new();
    for chunk in chunks {
        if let Some(file_id) = chunk {
            *file_id_to_size.entry(*file_id).or_insert(0) += 1;
        }
    }

    file_id_to_size
}

fn part_2(file_path: &str) -> Result<usize> {
    let disk_map = read_data(file_path)?;
    let mut chunks = parse_disk_map(disk_map);

    let file_id_to_size = get_file_id_to_size(&chunks);
    let mut decreasing_file_ids: Vec<u32> = file_id_to_size.keys().map(|key| *key).collect();
    decreasing_file_ids.sort_by(|a, b| b.cmp(a));

    for file_id in decreasing_file_ids {
        let file_size = file_id_to_size[&file_id];
        if let Some(start_free_space_index) = check_for_free_space(&chunks, file_size) {
            let first_file_size_pos = chunks.iter().position(|chunk| {
                if chunk.is_some() {
                    return chunk.unwrap() == file_id;
                }
                return false;
            });

            if first_file_size_pos.is_some()
                && first_file_size_pos.unwrap() > start_free_space_index
            {
                for i in 0..chunks.len() {
                    if chunks[i].is_some() && chunks[i].unwrap() == file_id {
                        chunks[i] = None;
                    }
                }
                for i in start_free_space_index..start_free_space_index + file_size as usize {
                    chunks[i as usize] = Some(file_id);
                }
            }
        }
    }

    let checksum: usize = chunks
        .iter()
        .enumerate()
        .filter_map(|(pos, file)| file.map(|id| pos * id as usize))
        .sum();

    Ok(checksum)
}

fn main() -> Result<()> {
    let part_1_result = part_1("input/data")?;
    println!(" -> Part one result: {part_1_result}");

    let part_2_result = part_2("input/data")?;
    println!(" -> Part two result: {part_2_result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_part_1_return_correct_result() {
        let part_1_result = part_1("input/test_data").expect("Should return correct value!");
        assert_eq!(part_1_result, 1928);
    }

    #[test]
    fn expect_part_2_return_correct_result() {
        let part_2_result = part_2("input/test_data").expect("Should return correct value!");
        assert_eq!(part_2_result, 2858);
    }
}
