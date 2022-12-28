interface Review {
    unitCode: string;
    unitName: string;
    unitDescription: string;
    unitRating: number;
    passedUnit: boolean;
    teachingYear: number;
    teachingSemester: string;
    datePublished: Date;
    lastUpdated: Date;
    approved: boolean;
    gradeAchieved: number;
    submittedBy: number;
}

export default Review;
