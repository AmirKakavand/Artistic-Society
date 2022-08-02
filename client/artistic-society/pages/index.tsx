import { useEffect, useState } from "react";
import styles from "../styles/Home.module.css";
import { Heading, Image } from "@chakra-ui/react";
import { useQuery } from "@tanstack/react-query";
import axios from "axios";

export default function Home() {
  const [response, setResponse] = useState(null);

  const getPainting = async () => {
    // generate random number between 435965 and 436184
    const randomNumber =
      Math.floor(Math.random() * (436184 - 435965 + 1)) + 435965;
    const apiResponse = await axios
      .get(
        `https://collectionapi.metmuseum.org/public/collection/v1/objects/${randomNumber}`
      )
      .then((res) => {
        console.log("response before setResponse: " + response)
        setResponse(res.data);
        console.log(response);
      })
      .catch((error) => {
        console.log(error);
      });
  };

  useEffect(() => {
    getPainting();
    console.log("in useEffect");
  }, []);
  // const { data, isLoading } = useQuery(["paintings"], getPainting);
  // console.log(data);
  return (
    <div className={styles.container}>
      <div className={styles.overlay}>
        <Heading
          as={"h1"}
          fontSize={{ base: "1.4rem", sm: "2rem", md: "3rem" }}
          mt={{ base: "2rem", md: "1rem" }}
        >
          Welcome to Artistic Society
        </Heading>

        {/* if we have a resopnse, show image */}
        {response ? (
          <Image src={response.primaryImageSmall} />
        ) : (
          <h1>nothing</h1>
        )}
      </div>
    </div>
  );
}
