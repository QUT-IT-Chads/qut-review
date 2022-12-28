interface Review {
    body: {
        Reviews: Array<{
            id: string;
            unit: {
                unit_code: string;
                unit_name: string;
                unit_description: string;
            };
            rating: number;
            passed_unit: boolean;
            teaching_period: {
                year: number;
                semester: {
                    semesterNumber: number;
                };
            };
            date_published: Date;
            last_updated: Date;
            approved: boolean;
            grade_achieved: number;
            fetchedAt: string;
            user: number;
        }>;
    };
}

export default Review;
