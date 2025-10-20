import * as React from "react";
import { twMerge } from "tailwind-merge";

export interface LabelProps extends React.LabelHTMLAttributes<HTMLLabelElement> {}

export function Label({ className, ...props }: LabelProps) {
  return (
    <label
      className={twMerge("text-sm font-medium text-neutral-800", className)}
      {...props}
    />
  );
}


