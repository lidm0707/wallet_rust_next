"use client";

import { useState } from "react";
import { ColumnDef } from "@tanstack/react-table";

// Define the shape of user data
export type User = { id: number; username: string };

export const columns: ColumnDef<User>[] = [
  {
    accessorKey: "id",
    header: "ID",
  },
  {
    accessorKey: "username",
    header: "NAME",
  },
  {
    header: "ACTION",
    cell: ({ row }) => {
      const receiverId = row.original.id; // Access the receiver's ID from the row data.

      return <SendMoneyButton receiverId={receiverId} />;
    },
  },
];

// SendMoneyButton component with modal
function SendMoneyButton({ receiverId }: { receiverId: number }) {
  const [isModalOpen, setModalOpen] = useState(false);
  const [amount, setAmount] = useState(0);

  const handleSend = () => {
    setModalOpen(false);
    handleSendMoney(receiverId, amount); // Example: sender ID 1
  };

  return (
    <>
      <button
        className="px-3 py-1 text-white bg-blue-500 rounded hover:bg-blue-600"
        onClick={() => setModalOpen(true)}
      >
        Send Money
      </button>

      {isModalOpen && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
          <div className="bg-white rounded-lg p-6 shadow-lg w-80">
            <h2 className="text-lg font-semibold mb-4">Set Amount</h2>
            <input
              type="number"
              className="w-full p-2 mb-4 border rounded"
              placeholder="Enter amount"
              value={amount}
              onChange={(e) => setAmount(parseFloat(e.target.value))}
            />
            <div className="flex justify-end space-x-2">
              <button
                className="px-4 py-2 bg-gray-300 rounded hover:bg-gray-400"
                onClick={() => setModalOpen(false)}
              >
                Cancel
              </button>
              <button
                className="px-4 py-2 text-white bg-blue-500 rounded hover:bg-blue-600"
                onClick={handleSend}
              >
                Send
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  );
}

async function handleSendMoney( receiverId: number, amount: number) {
  const token = localStorage.getItem("accessToken"); // Retrieve the token from localStorage

  if (!token) {
    alert("No token found. Please log in.");
    return;
  }

  const payload = {
    receiver_id: receiverId,
    amount: parseFloat(amount.toFixed(2)), // Ensures amount is a number with two decimal precision
  };
  try {
    const response = await fetch("http://localhost:8080/trans/send_money", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`, // Use the retrieved token
      },
      body: JSON.stringify(payload), // Convert the object to a JSON string
    });

    // Check if the response is not OK
    if (!response.ok) {
      
      const contentType = response.headers.get("Content-Type") || "";
      let errorMessage = "Failed to send money. Please try again.";

      const responseText = await response.text();

      if (contentType.includes("application/json")) {
        try {
          const errorData = JSON.parse(responseText);
          errorMessage = errorData.message || errorMessage;
        } catch (jsonParseError) {
          console.error("Failed to parse JSON:", jsonParseError);
        }
      } else {
        errorMessage = responseText;
      }

      console.error("Error sending money:", errorMessage);
      alert(errorMessage);
      return;
    }
  
   
    console.log("Money sent successfully:", response);
    alert(`Money sent to user ID ${receiverId} successfully!`);

  } catch (error) {
    console.error("Error occurred:", error);
    alert("An unexpected error occurred while sending money.");
  }
}

