/* tslint:disable */
/**
 * This file was automatically generated by json-schema-to-typescript.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run json-schema-to-typescript to regenerate this file.
 */

export type Semester = "Summer" | "Sem1" | "Sem2";

export interface NewReview {
  unit_code: string;
  rating: number;
  passed_unit: boolean;
  review_body: string;
  teaching_period: Semester;
  year_taken: number;
  grade_achieved?: number | null;
  user_id: string;
  [k: string]: unknown;
}
