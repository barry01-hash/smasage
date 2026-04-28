import type { ButtonHTMLAttributes, JSX } from "react";

export type ButtonVariant = "primary" | "secondary";

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: ButtonVariant;
  isLoading?: boolean;
  loadingLabel?: string;
}

export function Button({
  variant = "primary",
  isLoading = false,
  disabled,
  className,
  children,
  loadingLabel = "Loading",
  ...rest
}: ButtonProps): JSX.Element {
  const classes = ["btn", `btn-${variant}`, className].filter(Boolean).join(" ");

  return (
    <button
      className={classes}
      disabled={disabled || isLoading}
      aria-busy={isLoading || undefined}
      aria-label={isLoading ? loadingLabel : rest["aria-label"]}
      {...rest}
    >
      <span className={`btn-content ${isLoading ? "is-hidden" : ""}`}>{children}</span>
      {isLoading ? (
        <span className="btn-loader-wrap" aria-hidden="true">
          <svg className="btn-loader" viewBox="0 0 24 24" fill="none" focusable="false">
            <circle
              cx="12"
              cy="12"
              r="9"
              stroke="currentColor"
              strokeWidth="3"
              strokeLinecap="round"
              strokeDasharray="56.5"
              strokeDashoffset="18"
            />
          </svg>
        </span>
      ) : null}
    </button>
  );
}
