import { useEffect, useState } from "react";
import styles from "../styles/Home.module.css";
import { Box, Heading, Image, Spinner, Text } from "@chakra-ui/react";
import { useQuery } from "@tanstack/react-query";
import axios from "axios";

export default function Home() {
  const [response, setResponse] = useState(null);
  let counter = 0;

  const getPainting = async () => {
    // generate random number between 435965 and 436184
    const randomNumber =
      Math.floor(Math.random() * (436184 - 435965 + 1)) + 435965;
    const apiResponse = await axios
      .get(
        `https://collectionapi.metmuseum.org/public/collection/v1/objects/${randomNumber}`
      )
      .then((res) => {
        setResponse(res.data);
        console.log(response);
      })
      .catch((error) => {
        console.log(error);
        getPainting();
      });
  };

  useEffect(() => {
    if (counter === 0) {
      getPainting();
      counter++;
    }
    console.log("in useEffect");
  }, []);
  // const { data, isLoading } = useQuery(["paintings"], getPainting);
  // console.log(data);
  return (
    <>
      {/* <head>
      <title>Artistic Society</title>
      <link rel="icon" href="/favicon.ico" />
    </head> */}
      <Box
        height={"100vh"}
        backgroundImage={"/damavand.jpg"}
        backgroundSize={"cover"}
        backgroundPosition={"center"}
      >
        <Box
          width={"100%"}
          height={"100vh"}
          background={"rgba(10,10,10,0.2)"}
          backdropFilter={"blur(5px)"}
          display={"flex"}
          flexDirection={"column"}
          justifyContent={"flex-start"}
          alignItems={"center"}
          paddingX={{ base: "2rem", md: "0" }}
        >
          <Heading
            as={"h1"}
            fontSize={{ base: "2rem", md: "3rem" }}
            mt={{ base: "2rem", md: "1rem" }}
            textAlign={"center"}
            color={"whitesmoke"}
          >
            Welcome to{" "}
            {
              <Box display={{ base: "inline", md: "none" }}>
                <br />
              </Box>
            }{" "}
            Artistic Society
          </Heading>

          {/* if we have a resopnse, show image */}
          {response ? (
            <Image
              src={response.primaryImageSmall}
              marginTop={"3rem"}
              maxHeight={"xs"}
              className={styles.art}
              rounded={"md"}
            />
          ) : (
            <Box textAlign={"center"} marginTop={"4rem"}>
              <Spinner
                size={"xl"}
                color={"crimson"}
                thickness={"4px"}
                marginBottom={"1rem"}
                speed={"0.8s"}
              />
              <Text
                color={"whitesmoke"}
                fontSize={{ base: "2rem", md: "3rem" }}
              >
                Loading...
              </Text>
            </Box>
          )}
        </Box>
      </Box>
    </>
  );
}
