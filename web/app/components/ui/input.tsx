"use client";

import * as React from "react";
import { twMerge } from "tailwind-merge";

export interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {}

export const Input = React.forwardRef<HTMLInputElement, InputProps>(
  ({ className, ...props }, ref) => {
    return (
      <input
        ref={ref}
        className={twMerge(
          "flex h-11 w-full rounded-md bg-white px-3 text-sm shadow-sm ring-1 ring-inset ring-brand-100 placeholder:text-neutral-400 focus:outline-none focus:ring-2 focus:ring-[--ring]",
          className
        )}
        {...props}
      />
    );
  }
);
Input.displayName = "Input";


