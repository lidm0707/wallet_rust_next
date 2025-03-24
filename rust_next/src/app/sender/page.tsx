"use client";

import { useEffect, useState } from "react";
import { User, columns } from "./columns";
import { DataTable } from "./data-table";

async function getData(token: string): Promise<User[]> {
  const response = await fetch("http://localhost:8080/get_users", {
    method: "GET",
    headers: {
      Authorization: `Bearer ${token}`,
    },
  });

  if (!response.ok) {
    throw new Error("Failed to fetch data");
  }

  const result = await response.json();

  // Transform the fetched data to match the User type if needed
  return result.data.map((user: User) => ({
    id: user.id,
    username: user.username,
  }));
}

export default function Sender() {
  const [data, setData] = useState<User[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchData = async () => {
      const token = localStorage.getItem("accessToken");
      if (!token) {
        console.log("Token not found");
        setLoading(false);
        return;
      }

      try {
        const users = await getData(token);
        setData(users);
      } catch (error) {
        console.error("Error fetching data:", error);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, []);

  if (loading) {
    return (
      <div className="container mx-auto py-10">
        <p>Loading...</p>
      </div>
    );
  }

  return (
    <div className="container mx-auto py-10">
      <DataTable columns={columns} data={data} />
    </div>
  );
}
