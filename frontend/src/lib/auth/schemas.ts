import { z } from 'zod';

export const loginSchema = z.object({
	email: z.string().email('Please enter a valid email address'),
	password: z.string().min(1, 'Password is required')
});

export const registerSchema = z.object({
	username: z
		.string()
		.min(3, 'Username must be at least 3 characters')
		.max(50, 'Username must be less than 50 characters')
		.regex(/^[a-zA-Z0-9_-]+$/, 'Username can only contain letters, numbers, hyphens, and underscores'),
	email: z.string().email('Please enter a valid email address'),
	password: z
		.string()
		.min(8, 'Password must be at least 8 characters')
		.regex(/[A-Z]/, 'Password must contain at least one uppercase letter')
		.regex(/[a-z]/, 'Password must contain at least one lowercase letter')
		.regex(/[0-9]/, 'Password must contain at least one number'),
	confirmPassword: z.string()
}).refine((data) => data.password === data.confirmPassword, {
	message: "Passwords don't match",
	path: ['confirmPassword']
});

export type LoginFormData = z.infer<typeof loginSchema>;
export type RegisterFormData = z.infer<typeof registerSchema>;