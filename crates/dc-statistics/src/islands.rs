use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IslandKindStatistics {
    pub total_count: usize,
    pub actives_count: usize,
    pub upcomings_count: usize,
    pub pasts_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IslandsStatistics {
    pub total_count: usize,
    pub actives_count: usize,
    pub upcomings_count: usize,
    pub pasts_count: usize,
    pub maze: IslandKindStatistics,
    pub tower: IslandKindStatistics,
    pub fog: IslandKindStatistics,
    pub grid: IslandKindStatistics,
    pub heroic_races: IslandKindStatistics,
    pub runner: IslandKindStatistics,
    pub puzzle: IslandKindStatistics,
}

#[derive(Debug, Clone, Default)]
pub struct IslandSections<'a> {
    pub maze: &'a [Value],
    pub tower: &'a [Value],
    pub fog: &'a [Value],
    pub grid: &'a [Value],
    pub heroic_races: &'a [Value],
    pub runner: &'a [Value],
    pub puzzle: &'a [Value],
}

pub fn is_active_island(island: &Value, now_in_seconds: u64) -> bool {
    if let Some(availability) = island.get("availability") {
        if let (Some(from), Some(to)) = (
            availability.get("from").and_then(|v| v.as_u64()),
            availability.get("to").and_then(|v| v.as_u64()),
        ) {
            return now_in_seconds >= from && now_in_seconds <= to;
        }
    }

    if let (Some(start_ts), Some(end_ts)) = (
        island.get("start_ts").and_then(|v| v.as_u64()),
        island.get("end_ts").and_then(|v| v.as_u64()),
    ) {
        return now_in_seconds >= start_ts && now_in_seconds <= end_ts;
    }

    false
}

pub fn is_upcoming_island(island: &Value, now_in_seconds: u64) -> bool {
    if let Some(availability) = island.get("availability") {
        if let Some(from) = availability.get("from").and_then(|v| v.as_u64()) {
            return now_in_seconds < from;
        }
    }

    if let Some(start_ts) = island.get("start_ts").and_then(|v| v.as_u64()) {
        return now_in_seconds < start_ts;
    }

    false
}

pub fn is_past_island(island: &Value, now_in_seconds: u64) -> bool {
    if let Some(availability) = island.get("availability") {
        if let Some(to) = availability.get("to").and_then(|v| v.as_u64()) {
            return now_in_seconds > to;
        }
    }

    if let Some(end_ts) = island.get("end_ts").and_then(|v| v.as_u64()) {
        return now_in_seconds > end_ts;
    }

    false
}

pub fn generate_island_kind_statistics(
    islands: &[Value],
    now_in_seconds: u64,
) -> IslandKindStatistics {
    let mut actives_count = 0;
    let mut upcomings_count = 0;
    let mut pasts_count = 0;

    for island in islands {
        if is_active_island(island, now_in_seconds) {
            actives_count += 1;
        }
        if is_upcoming_island(island, now_in_seconds) {
            upcomings_count += 1;
        }
        if is_past_island(island, now_in_seconds) {
            pasts_count += 1;
        }
    }

    IslandKindStatistics {
        total_count: islands.len(),
        actives_count,
        upcomings_count,
        pasts_count,
    }
}

pub fn generate_islands_statistics_at(
    sections: &IslandSections,
    now_in_seconds: u64,
) -> IslandsStatistics {
    let maze = generate_island_kind_statistics(sections.maze, now_in_seconds);
    let tower = generate_island_kind_statistics(sections.tower, now_in_seconds);
    let fog = generate_island_kind_statistics(sections.fog, now_in_seconds);
    let grid = generate_island_kind_statistics(sections.grid, now_in_seconds);
    let heroic_races = generate_island_kind_statistics(sections.heroic_races, now_in_seconds);
    let runner = generate_island_kind_statistics(sections.runner, now_in_seconds);
    let puzzle = generate_island_kind_statistics(sections.puzzle, now_in_seconds);

    let total_count = maze.total_count
        + tower.total_count
        + fog.total_count
        + grid.total_count
        + heroic_races.total_count
        + runner.total_count
        + puzzle.total_count;

    let actives_count = maze.actives_count
        + tower.actives_count
        + fog.actives_count
        + grid.actives_count
        + heroic_races.actives_count
        + runner.actives_count
        + puzzle.actives_count;

    let upcomings_count = maze.upcomings_count
        + tower.upcomings_count
        + fog.upcomings_count
        + grid.upcomings_count
        + heroic_races.upcomings_count
        + runner.upcomings_count
        + puzzle.upcomings_count;

    let pasts_count = maze.pasts_count
        + tower.pasts_count
        + fog.pasts_count
        + grid.pasts_count
        + heroic_races.pasts_count
        + runner.pasts_count
        + puzzle.pasts_count;

    IslandsStatistics {
        total_count,
        actives_count,
        upcomings_count,
        pasts_count,
        maze,
        tower,
        fog,
        grid,
        heroic_races,
        runner,
        puzzle,
    }
}

pub fn generate_islands_statistics(sections: &IslandSections) -> IslandsStatistics {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    generate_islands_statistics_at(sections, now)
}
