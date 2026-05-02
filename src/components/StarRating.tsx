import React from "react";

interface Props {
  rating: number;
  onChange?: (rating: number) => void;
  readonly?: boolean;
}

export const StarRating: React.FC<Props> = ({ rating, onChange, readonly }) => {
  return (
    <div className="star-rating">
      {[1, 2, 3, 4, 5].map((star) => (
        <span
          key={star}
          className={`star ${star <= rating ? "filled" : ""} ${readonly ? "" : "interactive"}`}
          onClick={() => !readonly && onChange?.(star === rating ? 0 : star)}
        >
          {star <= rating ? "\u2605" : "\u2606"}
        </span>
      ))}
    </div>
  );
};
