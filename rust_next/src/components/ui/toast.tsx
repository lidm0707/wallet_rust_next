import { toast } from "react-toastify";

  export const showToastSuccess = (msg: string) => {
    toast.success(`success ${msg}!`, {
      position: 'top-right',
    });
  };

  export const showToastError = (msg: string) => {
    toast.error(`error ${msg}!`, {
      position: 'top-right',
    });
  };