use tuillez::formatter::FormatWithAsync;

use crate::interface::comp_arrow::ComparisonArrow;
use crate::tools::lookup::lookup_components::comp_arrow::LookupCompArrow;

pub struct TopRow<T> {
    pub ranking: usize,
    pub previous_ranking: Option<usize>,

    pub score: String,
    pub previous_score: Option<String>,

    pub element: T,
}

impl<T> TopRow<T> {
    pub fn rank_col(&self) -> String {
        match self.previous_ranking {
            None => format!("#{}", self.ranking),
            Some(previous_ranking) => format!(
                "{} #{} ({})",
                ComparisonArrow::lesser_is_better_inv(self.ranking, previous_ranking),
                self.ranking,
                previous_ranking
            ),
        }
    }

    pub fn score_col(&self) -> String {
        match &self.previous_score {
            None => format!("{}", self.score),
            Some(previous_score) => format!("{} ({})", self.score, previous_score),
        }
    }
}

impl<T, F> FormatWithAsync<F> for TopRow<T>
where
    T: FormatWithAsync<F> + Sync,
    F: Sync,
{
    type Error = <T as FormatWithAsync<F>>::Error;

    async fn format_with_async(&self, ft: &F) -> Result<String, Self::Error> {
        let inner = self.element.format_with_async(ft).await?;

        let comp_arrow = match self.previous_ranking {
            Some(prev_rank) => format!("{} ", LookupCompArrow::comp_asc(self.ranking, prev_rank)),
            None => "  ".to_string(),
        };

        Ok(format!(
            " {comp_arrow} #{:<3} | {:>4} | {inner:<60}",
            self.ranking, self.score
        ))
    }
}
