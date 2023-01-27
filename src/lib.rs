

enum TimeUnit {
    Hours,
    Days,
    Weeks,
    Months,
}

enum ProjectSize {
    Trivial,
    Small,
    Medium,
    Large,
    ExtraLarge,
    Epic
}

struct Team {
    team_id: u64,
    name: String,
    team_link: String,
    description: String,
    std_execution_time_interval: u16,
    std_execution_time_interval_unit: TimeUnit,
    complexity_points_per_time_interval: u16,
    complexity_points_per_time_interval_unit: TimeUnit,
}


struct TeamExecution {
    team_execution_id: u64,
    team_id: u64,
    start_date: u64,
    end_date: u64,
    execution_time_interval: u16,
    execution_time_interval_unit: TimeUnit,
    complexity_points: u64,
}

struct Project {
    project_id: u64,
    name: String,
    project_link: String,
    epic_ticket: String,
    description: String,
    version: String,
    last_updated: u64,
    dependencies: Vec<u64>,
    project_size: ProjectSize,
    project_type: String,
    execution_team: u64,
    complexity_points: u64,
}


struct User {
    user_id: u64,
    name: String,
    email: String,
    password_hash: String,
    user_type: UserType,
    last_login: u64,
    last_updated: u64,
}

enum UserType {
    Admin,
    User,
}

fn create_user(name: String, email: String, password: String, user_type: UserType) -> User {
    let user_id = 1;
    let last_login = 0;
    let last_updated = 0;
    User {
        user_id,
        name,
        email,
        password_hash: password,
        user_type,
        last_login,
        last_updated,
    }
}

fn create_team(name: String, team_link: String, description: String, std_execution_time_interval: u16, std_execution_time_interval_unit: TimeUnit, complexity_points_per_time_interval: u16, complexity_points_per_time_interval_unit: TimeUnit) -> Team {
    let team_id = 1;
    Team {
        team_id,
        name,
        team_link,
        description,
        std_execution_time_interval,
        std_execution_time_interval_unit,
        complexity_points_per_time_interval,
        complexity_points_per_time_interval_unit,
    }
}

fn update_team(team_id: u64, name: String, team_link: String, description: String, std_execution_time_interval: u16, std_execution_time_interval_unit: TimeUnit, complexity_points_per_time_interval: u16, complexity_points_per_time_interval_unit: TimeUnit) -> Team {
    Team {
        team_id,
        name,
        team_link,
        description,
        std_execution_time_interval,
        std_execution_time_interval_unit,
        complexity_points_per_time_interval,
        complexity_points_per_time_interval_unit,
    }
}


fn create_project(name: String, project_link: String, epic_ticket: String, description: String, version: String, last_updated: u64, dependencies: Vec<u64>, project_size: ProjectSize, project_type: String, execution_team: u64, complexity_points: u64) -> Project {
    let project_id = 1;
    Project {
        project_id,
        name,
        project_link,
        epic_ticket,
        description,
        version,
        last_updated,
        dependencies,
        project_size,
        project_type,
        execution_team,
        complexity_points,
    }
}

fn update_project(project_id: u64, name: String, project_link: String, epic_ticket: String, description: String, version: String, last_updated: u64, dependencies: Vec<u64>, project_size: ProjectSize, project_type: String, execution_team: u64, complexity_points: u64) -> Project {
    Project {
        project_id,
        name,
        project_link,
        epic_ticket,
        description,
        version,
        last_updated,
        dependencies,
        project_size,
        project_type,
        execution_team,
        complexity_points,
    }
}

fn create_team_execution(team_id: u64, start_date: u64, end_date: u64, execution_time_interval: u16, execution_time_interval_unit: TimeUnit, complexity_points: u64) -> TeamExecution {
    let team_execution_id = 1;
    TeamExecution {
        team_execution_id,
        team_id,
        start_date,
        end_date,
        execution_time_interval,
        execution_time_interval_unit,
        complexity_points,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user() {
        let user = create_user("test".to_string(), "fsdfs".to_string(), "fsdfs".to_string(), UserType::Admin);
        assert_eq!(user.user_id, 1);
    }
       
    #[test]
    fn test_create_team() {
        let team = create_team("test".to_string(), "fsdfs".to_string(), "fsdfs".to_string(), 1, TimeUnit::Hours, 1, TimeUnit::Hours);
        assert_eq!(team.team_id, 1);
    }

    #[test]
    fn test_update_team() {
        let team = update_team(1, "test".to_string(), "fsdfs".to_string(), "fsdfs".to_string(), 1, TimeUnit::Hours, 1, TimeUnit::Hours);
        assert_eq!(team.team_id, 1);
    }

    #[test]
    fn test_create_project() {
        let project = create_project("test".to_string(), "fsdfs".to_string(), "fsdfs".to_string(), "fsdfs".to_string(), "fsdfs".to_string(), 1, vec![1, 2, 3], ProjectSize::Small, "fsdfs".to_string(), 1, 1);
        assert_eq!(project.project_id, 1);
    }
    
    #[test]
    fn test_update_project() {
        let project = update_project(1, "test".to_string(), "fsdfs".to_string(), "fsdfs".to_string(), "fsdfs".to_string(), "fsdfs".to_string(), 1, vec![1, 2, 3], ProjectSize::Small, "fsdfs".to_string(), 1, 1);
        assert_eq!(project.project_id, 1);
    }

    #[test]
    fn test_create_team_execution() {
        let team_execution = create_team_execution(1, 1, 1, 1, TimeUnit::Hours, 1);
        assert_eq!(team_execution.team_execution_id, 1);
    }


}