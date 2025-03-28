"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { useRouter } from "next/navigation"; // Use Next.js Router
import "react-toastify/dist/ReactToastify.css";
import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import Link from "next/link";
import { showToastError, showToastSuccess } from "@/components/ui/toast";

// Schema validation
const formSchema = z.object({
  username: z.string().min(2, {
    message: "Username must be at least 2 characters.",
  }),
  password: z.string().min(6, {
    message: "Password must be at least 6 characters.",
  }),
});

export default function Home() {
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      username: "",
      password: "",
    },
  });

  const router = useRouter(); // Initialize router for navigation

  // Submit handler
  async function onSubmit(data: z.infer<typeof formSchema>) {
    try {
      const response = await fetch("http://localhost:8080/authen/login", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
      });
      if (response.ok) {
        const responseData = await response.json();
        console.log(responseData);
        const { access_token, refresh_token } = responseData.data;

        if (access_token && refresh_token) {
          localStorage.setItem("accessToken", access_token);
          localStorage.setItem("refreshToken", refresh_token);
          router.push("/sender")

        }
        await showToastSuccess("Login")
      } else {
        await showToastError("something went wrong")
      }
    } catch (error) {
      console.error("There was a problem with the fetch operation:", error)
      await showToastError(`${error}!`)
    }
  }
  return (
    <div className="flex justify-center items-center min-h-screen bg-gray-100">
      <Form {...form}>
        <form
          onSubmit={form.handleSubmit(onSubmit)}
          className="space-y-8 bg-white p-6 rounded-lg shadow-md"
        >
          {/* Username Field */}
          <FormField
            control={form.control}
            name="username"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Username</FormLabel>
                <FormControl>
                  <Input data-testid = "user"  placeholder="Enter your username" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />

          {/* Password Field */}
          <FormField
            control={form.control}
            name="password"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Password</FormLabel>
                <FormControl>
                  <Input data-testid = "password"  type="password" placeholder="Enter your password" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />

          {/* Register Link */}
          <div className="text-center">
            <Link
            data-testid = "register"
              href="/register"
              className="text-blue-600 hover:text-blue-800 font-medium underline"
            >
              Don't have an account? Register here.
            </Link>
          </div>

          {/* Submit Button */}
          <Button type="submit" className="w-full" data-testid = "submit" >
            Submit
          </Button>
        </form>
      </Form>
    </div>
  );
}
