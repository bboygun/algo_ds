// 207. 课程表
// 你这个学期必须选修 numCourses 门课程，记为 0 到 numCourses - 1 。
// 在选修某些课程之前需要一些先修课程。 先修课程按数组 prerequisites 给出，其中 prerequisites[i] = [ai, bi] ，表示如果要学习课程 ai 则 必须 先学习课程  bi 。
// 例如，先修课程对 [0, 1] 表示：想要学习课程 0 ，你需要先完成课程 1 。
// 请你判断是否可能完成所有课程的学习？如果可以，返回 true ；否则，返回 false 。

// 示例 1：
// 输入：numCourses = 2, prerequisites = [[1,0]]
// 输出：true
// 解释：总共有 2 门课程。学习课程 1 之前，你需要完成课程 0 。这是可能的。

// 示例 2：
// 输入：numCourses = 2, prerequisites = [[1,0],[0,1]]
// 输出：false
// 解释：总共有 2 门课程。学习课程 1 之前，你需要先完成​课程 0 ；并且学习课程 0 之前，你还应先完成课程 1 。这是不可能的。

use std::collections::{HashMap, HashSet};

fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut in_degree: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut out_degree: HashMap<i32, HashSet<i32>> = HashMap::new();

    for pair in prerequisites {
        let cur = pair[0];
        let dependency = pair[1];

        let mut cur_in = in_degree.entry(cur).or_insert(HashSet::new());
        cur_in.insert(dependency);

        let mut dependency_out = out_degree.entry(dependency).or_insert(HashSet::new());
        dependency_out.insert(cur);
    }

    for i in 0..num_courses {
        in_degree.entry(i).or_insert(HashSet::new());
        out_degree.entry(i).or_insert(HashSet::new());
    }

    let mut cnt = num_courses;
    loop {
        let mut to_do = vec![];
        for (&cur, dependency_list) in in_degree.iter() {
            if dependency_list.is_empty() {
                to_do.push(cur);
            }
        }
        if to_do.is_empty() {
            break;
        }
        for cur in to_do {
            cnt -= 1;
            in_degree.remove(&cur);

            for next in out_degree.get_mut(&cur).unwrap().iter() {
                in_degree.get_mut(next).unwrap().remove(&cur);
            }
        }
    }
    cnt == 0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_finish_test() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0]];
        assert!(can_finish(num_courses, prerequisites));

        let num_courses = 2;
        let prerequisites = vec![vec![1, 0], vec![0, 1]];
        assert!(!can_finish(num_courses, prerequisites));
    }
}
